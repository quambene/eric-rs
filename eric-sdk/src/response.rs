use crate::error_code::ErrorCode;
use eric_bindings::{
    EricReturnBufferApi, EricRueckgabepufferErzeugen, EricRueckgabepufferFreigeben,
    EricRueckgabepufferInhalt,
};
use std::ffi::CStr;

/// The outcome of an [`Eric::validate`](crate::Eric::validate) call.
///
/// ERIC distinguishes three outcomes for a validation run:
///
/// - [`Valid`](ValidationOutcome::Valid): the document passed all checks (`ERIC_OK`).
/// - [`Invalid`](ValidationOutcome::Invalid): plausibility errors were found
///   (`ERIC_GLOBAL_PRUEF_FEHLER`, 610001002). The inner [`EricResponse`] carries
///   the field-level error details in `validation_response`.
/// - [`Hints`](ValidationOutcome::Hints): only informational hints
///   (`ERIC_GLOBAL_HINWEISE`, 610001003). The document is technically acceptable
///   but has warnings.
///
/// All three variants wrap the raw [`EricResponse`] so callers can inspect the
/// full XML regardless of the outcome.
#[derive(Debug)]
pub enum ValidationOutcome {
    Valid(EricResponse),
    Invalid(EricResponse),
    Hints(EricResponse),
}

impl ValidationOutcome {
    /// Returns `true` only when validation passed without errors or hints.
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationOutcome::Valid(_))
    }

    /// Borrows the inner [`EricResponse`] regardless of the outcome variant.
    pub fn response(&self) -> &EricResponse {
        match self {
            ValidationOutcome::Valid(r)
            | ValidationOutcome::Invalid(r)
            | ValidationOutcome::Hints(r) => r,
        }
    }

    /// Consumes the outcome and returns the inner [`EricResponse`].
    pub fn into_response(self) -> EricResponse {
        match self {
            ValidationOutcome::Valid(r)
            | ValidationOutcome::Invalid(r)
            | ValidationOutcome::Hints(r) => r,
        }
    }
}

/// A structure which summarizes the response from the Eric instance.
#[derive(Debug)]
pub struct EricResponse {
    /// The error code returned by the Eric instance.
    pub error_code: i32,
    /// The response when validating an XML file.
    pub validation_response: String,
    /// The response when an XML file is send to the tax authorities.
    pub server_response: String,
}

impl EricResponse {
    pub fn new(error_code: i32, validation_response: String, server_response: String) -> Self {
        Self {
            error_code,
            validation_response,
            server_response,
        }
    }
}

pub struct ResponseBuffer {
    ctx: *mut EricReturnBufferApi,
}

impl ResponseBuffer {
    pub fn new() -> Result<Self, anyhow::Error> {
        let response_buffer = unsafe { EricRueckgabepufferErzeugen() };

        Ok(ResponseBuffer {
            ctx: response_buffer,
        })
    }

    pub fn as_ptr(&self) -> *mut EricReturnBufferApi {
        self.ctx
    }

    pub fn read(&self) -> Result<&str, anyhow::Error> {
        let buffer = unsafe {
            let ptr = EricRueckgabepufferInhalt(self.ctx);
            CStr::from_ptr(ptr)
        };

        Ok(buffer.to_str()?)
    }
}

impl Drop for ResponseBuffer {
    fn drop(&mut self) {
        println!("Cleaning up response buffer");

        let error_code = unsafe { EricRueckgabepufferFreigeben(self.ctx) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => (),
            error_code => panic!("Can't drop reponse buffer: {}", error_code),
        }
    }
}
