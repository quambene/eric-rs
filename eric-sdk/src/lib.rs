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
    Validate = 1 << 1,
    Send = 1 << 2,
    Print = 1 << 5,
    SendAndPrint = 1 << 2 | 1 << 5,
    #[allow(dead_code)]
    CheckHints = 1 << 7,
    #[allow(dead_code)]
    ValidateWithoutDate = 1 << 8,
}

impl ProcessingFlag {
    pub fn into_u32(self) -> u32 {
        self as u32
    }
}

pub(crate) enum Preview {
    Yes = 1,
    No = 0,
}
