use crate::ValidationReport;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EricError {
    #[error("API Error (code={code}): {message}")]
    ApiError {
        code: i32,
        message: String,
        validation_response: String,
        server_response: String,
    },
    /// Unstructured catch all for internal errors
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl EricError {
    /// Returns the ERiC error code if this is an API error.
    pub fn code(&self) -> Option<i32> {
        match self {
            Self::ApiError { code, .. } => Some(*code),
            Self::Internal(_) => None,
        }
    }

    /// Returns the raw validation XML returned by ERiC.
    pub fn validation_response(&self) -> Option<&str> {
        match self {
            Self::ApiError {
                validation_response,
                ..
            } => Some(validation_response.as_str()),
            Self::Internal(_) => None,
        }
    }

    /// Returns the raw server XML returned by ERiC.
    pub fn server_response(&self) -> Option<&str> {
        match self {
            Self::ApiError {
                server_response, ..
            } => Some(server_response.as_str()),
            Self::Internal(_) => None,
        }
    }

    /// Parses the validation XML into a structured report.
    pub fn validation_report(&self) -> Result<Option<ValidationReport>, EricError> {
        let Some(validation_response) = self.validation_response() else {
            return Ok(None);
        };

        ValidationReport::parse(validation_response).map(Some)
    }
}
