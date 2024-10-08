use crate::error_code::ErrorCode;
use eric_bindings::{
    EricReturnBufferApi, EricRueckgabepufferErzeugen, EricRueckgabepufferFreigeben,
    EricRueckgabepufferInhalt,
};
use std::ffi::CStr;

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
