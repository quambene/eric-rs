use crate::{
    config::{CertificateConfig, PrintConfig},
    error_code::ErrorCode,
    response::{EricResponse, ResponseBuffer},
    utils::ToCString,
    ProcessingFlag,
};
use anyhow::{anyhow, Context};
use eric_bindings::{EricBearbeiteVorgang, EricBeende, EricDekodiereDaten, EricInitialisiere};
use std::{
    env::{self},
    path::Path,
    ptr,
};

/// A structure to manage the Eric instance from the shared C library.
///
/// Use [`Eric::new`] to initialize Eric. Closes Eric when dropped.
pub struct Eric;

impl Eric {
    /// Initializes a single-threaded Eric instance.
    pub fn new(log_path: &Path) -> Result<Self, anyhow::Error> {
        println!("Initializing eric");

        let plugin_path =
            env::var("PLUGIN_PATH").context("Missing environment variable 'PLUGIN_PATH'")?;
        let plugin_path = Path::new(&plugin_path);

        println!("Setting plugin path '{}'", plugin_path.display());
        println!("Setting log path '{}'", log_path.display());
        println!("Logging to '{}'", log_path.join("eric.log").display());

        let plugin_path = plugin_path.try_to_cstring()?;
        let log_path = log_path.try_to_cstring()?;

        let error_code = unsafe { EricInitialisiere(plugin_path.as_ptr(), log_path.as_ptr()) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => Ok(Eric),
            error_code => Err(anyhow!("Can't init eric: {}", error_code)),
        }
    }

    /// Validates an XML file for a specific taxonomy.
    ///
    /// Optionally, a confirmation is printed to `pdf_path`.
    pub fn validate(
        &self,
        xml: String,
        taxonomy_type: &str,
        taxonomy_version: &str,
        pdf_path: Option<&str>,
    ) -> Result<EricResponse, anyhow::Error> {
        let processing_flag: ProcessingFlag;
        let type_version = format!("{}_{}", taxonomy_type, taxonomy_version);
        let print_config = if let Some(pdf_path) = pdf_path {
            processing_flag = ProcessingFlag::Print;
            Some(PrintConfig::new(pdf_path, &processing_flag)?)
        } else {
            processing_flag = ProcessingFlag::Validate;
            None
        };
        Self::process(xml, type_version, processing_flag, print_config, None, None)
    }

    /// Sends an XML file for a specific taxonomy to the tax authorities.
    ///
    /// The Elster certificate is provided via environment variables
    /// `CERTIFICATE_PATH` and `CERTIFICATE_PASSWORD`.
    ///
    /// Optionally, a confirmation is printed to `pdf_path`.
    pub fn send(
        &self,
        xml: String,
        taxonomy_type: &str,
        taxonomy_version: &str,
        pdf_path: Option<&str>,
    ) -> Result<EricResponse, anyhow::Error> {
        let certificate_path = env::var("CERTIFICATE_PATH")
            .context("Missing environment variable 'CERTIFICATE_PATH'")?;
        let certificate_password = env::var("CERTIFICATE_PASSWORD")
            .context("Missing environment variable 'CERTIFICATE_PASSWORD'")?;

        let processing_flag: ProcessingFlag;
        let type_version = format!("{}_{}", taxonomy_type, taxonomy_version);
        let print_config = if let Some(pdf_path) = pdf_path {
            processing_flag = ProcessingFlag::SendAndPrint;
            Some(PrintConfig::new(pdf_path, &processing_flag)?)
        } else {
            processing_flag = ProcessingFlag::Send;
            None
        };
        let certificate_config = CertificateConfig::new(&certificate_path, &certificate_password)?;
        Self::process(
            xml,
            type_version,
            processing_flag,
            print_config,
            Some(certificate_config),
            None,
        )
    }

    #[allow(dead_code)]
    fn decrypt(
        &self,
        encrypted_file: &str,
        certificate_config: CertificateConfig,
    ) -> Result<i32, anyhow::Error> {
        let encrypted_data = encrypted_file.try_to_cstring()?;
        let response_buffer = ResponseBuffer::new()?;

        let error_code = unsafe {
            EricDekodiereDaten(
                certificate_config.certificate.handle,
                certificate_config.password.as_ptr(),
                encrypted_data.as_ptr(),
                response_buffer.as_ptr(),
            )
        };

        Ok(error_code)
    }

    fn process(
        xml: String,
        type_version: String,
        processing_flag: ProcessingFlag,
        print_config: Option<PrintConfig>,
        certificate_config: Option<CertificateConfig>,
        transfer_code: Option<u32>,
    ) -> Result<EricResponse, anyhow::Error> {
        println!("Processing xml file");

        match processing_flag {
            ProcessingFlag::Validate => println!("Validating xml file"),
            ProcessingFlag::Print => println!("Validating xml file"),
            ProcessingFlag::Send => println!("Sending xml file"),
            ProcessingFlag::SendAndPrint => println!("Send and print"),
            ProcessingFlag::CheckHints => println!("Check hints"),
            ProcessingFlag::ValidateWithoutDate => println!("Validate without release date (German: Validiere ohne Freigabadatum)"),
        }

        let xml = xml.try_to_cstring()?;
        let type_version = type_version.try_to_cstring()?;

        // Transfer_code should be NULL except for data retrieval; if
        // transfer_code is not NULL in the other cases, it will be ignored
        let transfer_code = match transfer_code {
            Some(mut code) => &mut code,
            None => ptr::null::<u32>() as *mut u32,
        };

        match &print_config {
            Some(print_config) => println!(
                "Printing confirmation to file '{}'",
                print_config.pdf_path.to_str()?
            ),
            None => (),
        }

        let validation_response_buffer = ResponseBuffer::new()?;
        let server_response_buffer = ResponseBuffer::new()?;

        let error_code = unsafe {
            EricBearbeiteVorgang(
                xml.as_ptr(),
                type_version.as_ptr(),
                processing_flag.into_u32(),
                // SAFETY: match a reference of print_config; otherwise
                // print_config is moved, and print_parameter.as_ptr() would be
                // dangling
                match &print_config {
                    Some(el) => el.print_parameter.as_ptr(),
                    None => ptr::null(),
                },
                // SAFETY: match a reference of certificate_config; otherwise
                // certificate_config is moved, and
                // certificate_parameter.as_ptr() would be dangling
                match &certificate_config {
                    Some(el) => el.certificate_parameter.as_ptr(),
                    None => ptr::null(),
                },
                transfer_code,
                validation_response_buffer.as_ptr(),
                server_response_buffer.as_ptr(),
            )
        };

        // TODO: EricHoleFehlerText() for error code

        let transfer_code = unsafe { transfer_code.as_ref() };

        if let Some(code) = transfer_code {
            println!("Transfer code: {}", code)
        }

        let validation_response = validation_response_buffer.read()?;
        // TODO: parse server response via EricGetErrormessagesFromXMLAnswer()
        let server_response = server_response_buffer.read()?;
        let response = EricResponse::new(
            error_code,
            validation_response.to_string(),
            server_response.to_string(),
        );

        Ok(response)
    }
}

impl Drop for Eric {
    fn drop(&mut self) {
        println!("Closing eric");

        // TODO: implement EricEntladePlugins

        let error_code = unsafe { EricBeende() };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => (),
            error_code => panic!("Can't close eric: {}", error_code),
        }
    }
}
