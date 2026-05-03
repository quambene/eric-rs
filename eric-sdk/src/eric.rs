use crate::{
    config::{CertificateConfig, PrintConfig},
    error::EricError,
    error_code::ErrorCode,
    response::{EricResponse, ResponseBuffer},
    utils::ToCString,
    ProcessingFlag,
};
use anyhow::{anyhow, Context};
use eric_bindings::{
    EricBearbeiteVorgang, EricBeende, EricCheckXML, EricDekodiereDaten, EricEntladePlugins,
    EricHoleFehlerText, EricInitialisiere,
};
use std::{path::Path, ptr};
use tracing::{debug, error, info};

/// A structure to manage the Eric instance from the shared C library.
///
/// Use [`Eric::new`] to initialize Eric. Closes Eric when dropped.
pub struct Eric;

impl Eric {
    /// Initializes a single-threaded Eric instance.
    ///
    /// If `log_path` is `None`, the system directory for temporary files is
    /// used. If `plugin_path` is `None`, the path to the shared C library is
    /// used.
    pub fn new(log_path: Option<&Path>, plugin_path: Option<&Path>) -> Result<Self, EricError> {
        info!("Initializing eric");

        if let Some(log_path) = log_path {
            info!(log_path = %log_path.display(), "Setting log path");
            info!(log_file = %log_path.join("eric.log").display(), "Logging to file");
        } else {
            info!("No log path provided, using ERiC default temporary directory");
        }

        if let Some(plugin_path) = plugin_path {
            info!(plugin_path = %plugin_path.display(), "Setting plugin path");
        } else {
            info!("No plugin path provided, using ERiC default plugin directory");
        }

        // SAFETY: `plugin_path_cstring` must outlive `plugin_ptr`.
        let plugin_path_cstring = plugin_path
            .map(|plugin_path| plugin_path.try_to_cstring())
            .transpose()
            .context("failed to convert plugin path to CString")?;
        let plugin_ptr = plugin_path_cstring
            .as_deref()
            .map_or(ptr::null(), |cstr| cstr.as_ptr());

        // SAFETY: `log_path_cstring` must outlive `log_path_ptr`.
        let log_path_cstring = log_path
            .map(|path| path.try_to_cstring())
            .transpose()
            .context("failed to convert log path to CString")?;
        let log_path_ptr = log_path_cstring
            .as_deref()
            .map_or(ptr::null(), |cstr| cstr.as_ptr());

        let error_code = unsafe { EricInitialisiere(plugin_ptr, log_path_ptr) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => Ok(Eric),
            error_code => Err(EricError::Internal(anyhow!(
                "Can't init eric: {}",
                error_code
            ))),
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
    ) -> Result<EricResponse, EricError> {
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
    /// The Elster certificate needs to be provided at path `certificate_path`
    /// with password `certificate_password`.
    ///
    /// Optionally, a confirmation is printed to `pdf_path`.
    pub fn send(
        &self,
        xml: String,
        taxonomy_type: &str,
        taxonomy_version: &str,
        certificate_path: &Path,
        certificate_password: &str,
        pdf_path: Option<&str>,
    ) -> Result<EricResponse, EricError> {
        let certificate_path = certificate_path
            .to_str()
            .context("failed to convert path to string")?;
        let processing_flag: ProcessingFlag;
        let type_version = format!("{}_{}", taxonomy_type, taxonomy_version);
        let print_config = if let Some(pdf_path) = pdf_path {
            processing_flag = ProcessingFlag::SendAndPrint;
            Some(PrintConfig::new(pdf_path, &processing_flag)?)
        } else {
            processing_flag = ProcessingFlag::Send;
            None
        };
        let certificate_config = CertificateConfig::new(certificate_path, certificate_password)?;
        Self::process(
            xml,
            type_version,
            processing_flag,
            print_config,
            Some(certificate_config),
            None,
        )
    }

    /// Validates an XML file against the schema of a specific taxonomy.
    ///
    /// This is a schema-only check via ERiC's `EricCheckXML` and does not
    /// execute the full validation/send pipeline of [`Eric::validate`] or
    /// [`Eric::send`].
    ///
    /// Note that ERiC may report unsupported data types/versions for this
    /// API function.
    pub fn check_xml(
        &self,
        xml: String,
        taxonomy_type: &str,
        taxonomy_version: &str,
    ) -> Result<EricResponse, EricError> {
        let type_version = format!("{}_{}", taxonomy_type, taxonomy_version);
        let xml = xml.try_to_cstring()?;
        let type_version = type_version.try_to_cstring()?;

        let validation_response_buffer = ResponseBuffer::new()?;

        let error_code = unsafe {
            EricCheckXML(
                xml.as_ptr(),
                type_version.as_ptr(),
                validation_response_buffer.as_ptr(),
            )
        };

        let validation_response = validation_response_buffer.read()?;

        if error_code != ErrorCode::ERIC_OK as i32 {
            let response_buffer = ResponseBuffer::new()?;
            unsafe {
                EricHoleFehlerText(error_code, response_buffer.as_ptr());
            }
            let error_text = response_buffer.read()?;
            return Err(EricError::ApiError {
                code: error_code,
                message: error_text.to_string(),
                validation_response: validation_response.to_string(),
                server_response: String::new(),
            });
        }

        Ok(EricResponse::new(
            error_code,
            validation_response.to_string(),
            String::new(),
        ))
    }

    /// Returns the error text for a specific error code.
    pub fn get_error_text(&self, error_code: i32) -> Result<String, EricError> {
        let response_buffer = ResponseBuffer::new()?;

        unsafe {
            EricHoleFehlerText(error_code, response_buffer.as_ptr());
        }

        Ok(response_buffer.read()?.to_string())
    }

    #[allow(dead_code)]
    fn decrypt(
        &self,
        encrypted_file: &str,
        certificate_config: CertificateConfig,
    ) -> Result<i32, EricError> {
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
    ) -> Result<EricResponse, EricError> {
        debug!("Processing xml file");

        match processing_flag {
            ProcessingFlag::Validate => debug!("Validating xml file"),
            ProcessingFlag::Print => debug!("Validating xml file"),
            ProcessingFlag::Send => debug!("Sending xml file"),
            ProcessingFlag::SendAndPrint => debug!("Send and print"),
            ProcessingFlag::CheckHints => debug!("Check hints"),
            ProcessingFlag::ValidateWithoutDate => debug!("Validate without release date"),
        }

        let xml = xml.try_to_cstring()?;
        let type_version = type_version.try_to_cstring()?;

        // Transfer_code should be NULL except for data retrieval; if
        // transfer_code is not NULL in the other cases, it will be ignored.
        // SAFETY: `transfer_code_storage` must outlive `transfer_code_ptr`.
        let mut transfer_code_storage = transfer_code;
        let transfer_code_ptr: *mut u32 = transfer_code_storage
            .as_mut()
            .map_or(ptr::null_mut(), |c| c as *mut u32);

        if let Some(print_config) = &print_config {
            info!(
                pdf_path = %print_config
                    .pdf_path
                    .to_str()
                    .context("failed to convert path to string")?,
                "Printing confirmation to file"
            )
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
                    Some(config) => config.certificate_parameter.as_ptr(),
                    None => ptr::null(),
                },
                transfer_code_ptr,
                validation_response_buffer.as_ptr(),
                server_response_buffer.as_ptr(),
            )
        };

        let transfer_code = unsafe { transfer_code_ptr.as_ref() };

        if let Some(code) = transfer_code {
            debug!(transfer_code = %code, "Transfer code received")
        }

        let validation_response = validation_response_buffer.read()?;
        // TODO: parse server response via EricGetErrormessagesFromXMLAnswer()
        let server_response = server_response_buffer.read()?;

        if error_code != ErrorCode::ERIC_OK as i32 {
            let response_buffer = ResponseBuffer::new()?;

            unsafe {
                EricHoleFehlerText(error_code, response_buffer.as_ptr());
            }

            let error_text = response_buffer.read()?;

            return Err(EricError::ApiError {
                code: error_code,
                message: error_text.to_string(),
                validation_response: validation_response.to_string(),
                server_response: server_response.to_string(),
            });
        }

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
        info!("Closing eric");

        let error_code = unsafe { EricEntladePlugins() };

        if error_code != ErrorCode::ERIC_OK as i32 {
            error!(error_code = %error_code, "Error while unloading plugins");
        }

        let error_code = unsafe { EricBeende() };

        if error_code != ErrorCode::ERIC_OK as i32 {
            error!(error_code = %error_code, "Can't close eric");
        }
    }
}
