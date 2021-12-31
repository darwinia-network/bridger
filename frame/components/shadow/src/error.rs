use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum ShadowComponentError {
    #[error("Internal server error: {0}")]
    InternalServer(String),
}
