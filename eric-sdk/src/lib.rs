mod certificate;
mod config;
mod eric;
mod error_code;
mod response;
mod utils;

pub use eric::Eric;
pub use error_code::ErrorCode;
pub use response::EricResponse;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProcessingFlag {
    Validate = 2,
    Send = 4,
    Print = 32,
    SendAndPrint = 36,
    #[allow(dead_code)]
    CheckHints = 128,
}

impl ProcessingFlag {
    pub fn into_u32(&self) -> u32 {
        *self as u32
    }
}

pub(crate) enum Preview {
    Yes = 1,
    No = 0,
}
