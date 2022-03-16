use thiserror::Error as ThisError;

/// Shadow component result
pub type ShadowComponentReuslt<T> = Result<T, ShadowComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum ShadowComponentError {
    #[error("Internal server error: {0}")]
    InternalServer(String),
}
