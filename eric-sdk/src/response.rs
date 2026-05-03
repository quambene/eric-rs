use crate::error_code::ErrorCode;
use anyhow::anyhow;
use eric_bindings::{
    EricReturnBufferApi, EricRueckgabepufferErzeugen, EricRueckgabepufferFreigeben,
    EricRueckgabepufferInhalt,
};
use std::ffi::CStr;
use tracing::debug;

/// Shared payload for `EricResponse` and `EricError` returned by ERiC API
/// calls.
#[derive(Debug, Clone)]
pub struct EricApiPayload {
    /// The response when validating an XML file.
    pub validation_response: String,
    /// The response when an XML file is sent to the tax authorities.
    pub server_response: String,
}

impl EricApiPayload {
    pub fn new(validation_response: String, server_response: String) -> Self {
        Self {
            validation_response,
            server_response,
        }
    }
}

/// A structure which summarizes the response from the Eric instance.
///
/// Usually returned payload can be ignored when the API call was successful and
/// looks like this:
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <EricBearbeiteVorgang xmlns="http://www.elster.de/EricXML/1.1/EricBearbeiteVorgang">
///     <Erfolg/>
/// </EricBearbeiteVorgang>
/// ```
#[derive(Debug)]
pub struct EricResponse {
    /// XML payload returned by ERiC.
    pub payload: EricApiPayload,
}

impl EricResponse {
    pub fn new(payload: EricApiPayload) -> Self {
        Self { payload }
    }

    pub fn validation_response(&self) -> &str {
        self.payload.validation_response.as_str()
    }

    pub fn server_response(&self) -> &str {
        self.payload.server_response.as_str()
    }
}

/// A wrapper type for the response buffer of the Eric instance.
pub struct ResponseBuffer {
    ctx: *mut EricReturnBufferApi,
}

impl ResponseBuffer {
    pub fn new() -> Result<Self, anyhow::Error> {
        let response_buffer = unsafe { EricRueckgabepufferErzeugen() };

        if response_buffer.is_null() {
            return Err(anyhow!("EricRueckgabepufferErzeugen returned null"));
        }

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

            if ptr.is_null() {
                return Err(anyhow!("EricRueckgabepufferInhalt returned null"));
            }

            CStr::from_ptr(ptr)
        };

        Ok(buffer.to_str()?)
    }
}

impl Drop for ResponseBuffer {
    fn drop(&mut self) {
        debug!("Cleaning up response buffer");

        let error_code = unsafe { EricRueckgabepufferFreigeben(self.ctx) };

        match error_code {
            x if x == ErrorCode::ERIC_OK as i32 => (),
            error_code => panic!("Can't drop reponse buffer: {}", error_code),
        }
    }
}
