use crate::error_code::ErrorCode;
use anyhow::anyhow;
use eric_bindings::{EricCloseHandleToCertificate, EricGetHandleToCertificate};
use std::{ffi::CStr, ptr};

pub struct Certificate {
    pub handle: u32,
}

impl Certificate {
    pub fn new(path: &CStr) -> Result<Self, anyhow::Error> {
        println!("Preparing certificate '{}'", path.to_str()?);

        let mut handle = 0;
        let pin_support = ptr::null::<u32>() as *mut u32;

        // SAFETY: path.as_ptr() is not dangling as path is allocated in struct CertificateConfig and path is not moved as a reference to the CString is given
        let error_code =
            unsafe { EricGetHandleToCertificate(&mut handle, pin_support, path.as_ptr()) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => Ok(Certificate { handle }),
            error_code => Err(anyhow!("Can't create certificate: {}", error_code)),
        }
    }

    // TODO: check validity of certificate
    // unsafe { EricHoleZertifikatseigenschaften() }
}

impl Drop for Certificate {
    fn drop(&mut self) {
        let error_code = unsafe { EricCloseHandleToCertificate(self.handle) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => (),
            error_code => panic!("Can't drop certificate handle: {}", error_code),
        }
    }
}
