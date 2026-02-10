use thiserror::Error;

#[derive(Debug, Error)]
pub enum EricError {
    #[error("API Error (code={code}): {message}, validation={validation_response}, server={server_response}")]
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
