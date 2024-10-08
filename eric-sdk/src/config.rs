use crate::{certificate::Certificate, utils::ToCString, Preview, ProcessingFlag};
use eric_bindings::{eric_druck_parameter_t, eric_verschluesselungs_parameter_t, EricPdfCallback};
use std::{
    ffi::{c_void, CStr, CString},
    path::Path,
    ptr,
};

pub struct CertificateConfig {
    // allocate path as CString
    _path: CString,
    // allocate password as CString
    pub(crate) password: CString,
    pub(crate) certificate: Certificate,
    pub(crate) certificate_parameter: CertificateParameter,
}

impl CertificateConfig {
    pub fn new(path: &str, password: &str) -> Result<Self, anyhow::Error> {
        let path = Path::new(path).try_to_cstring()?;
        let password = password.try_to_cstring()?;
        let certificate = Certificate::new(&path)?;
        let certificate_parameter = CertificateParameter::new(&certificate, &password);

        Ok(Self {
            _path: path,
            password,
            certificate,
            certificate_parameter,
        })
    }
}

// CertificateParameter is a wrapper type to keep C memory alignment for eric_verschluesselungs_parameter_t
#[derive(Debug)]
#[repr(C)]
pub(crate) struct CertificateParameter {
    inner: eric_verschluesselungs_parameter_t,
}

impl CertificateParameter {
    pub(crate) fn new(certificate: &Certificate, password: &CStr) -> Self {
        let certificate_parameter = eric_verschluesselungs_parameter_t {
            // SAFETY: password.as_ptr() is not dangling as password is allocated in struct CertificateConfig and password is not moved as a reference to the CString is given
            pin: password.as_ptr(),
            version: 3,
            zertifikatHandle: certificate.handle,
            abrufCode: ptr::null(),
        };

        Self {
            inner: certificate_parameter,
        }
    }

    pub fn as_ptr(&self) -> *const eric_verschluesselungs_parameter_t {
        &self.inner
    }
}

#[derive(Debug)]
pub struct PrintConfig {
    // allocate pdf path as CString
    pub(crate) pdf_path: CString,
    pub(crate) print_parameter: PrintParameter,
}

impl PrintConfig {
    pub fn new(pdf_path: &str, processing_flag: &ProcessingFlag) -> Result<Self, anyhow::Error> {
        let pdf_path = Path::new(pdf_path).try_to_cstring()?;
        let print_parameter = PrintParameter::new(&pdf_path, processing_flag);

        Ok(Self {
            pdf_path,
            print_parameter,
        })
    }
}

// PrintParameter is a wrapper type to keep C memory alignment for eric_druck_parameter_t
#[derive(Debug)]
#[repr(C)]
pub(crate) struct PrintParameter {
    inner: eric_druck_parameter_t,
}

impl PrintParameter {
    pub(crate) fn new(pdf_path: &CStr, processing_flag: &ProcessingFlag) -> Self {
        let mut user_data: EricPdfCallback = None;
        let user_data_ptr: *mut c_void = &mut user_data as *mut _ as *mut c_void;
        let print_parameter = eric_druck_parameter_t {
            version: 4,
            vorschau: match processing_flag {
                ProcessingFlag::Validate => Preview::Yes as u32,
                ProcessingFlag::Print => Preview::Yes as u32,
                _ => Preview::No as u32,
            },
            ersteSeite: 0,
            duplexDruck: 0,
            // SAFETY: pdf_path.as_ptr() is not dangling as pdf_path is
            // allocated in struct PrintConfig and pdf_path is not moved as a
            // reference to the CString is given.
            pdfName: pdf_path.as_ptr(),
            fussText: ptr::null(),
            pdfCallback: None,
            pdfCallbackBenutzerdaten: user_data_ptr,
        };

        Self {
            inner: print_parameter,
        }
    }

    pub fn as_ptr(&self) -> *const eric_druck_parameter_t {
        &self.inner
    }
}
