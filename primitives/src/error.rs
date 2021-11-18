#![allow(missing_docs)]

use thiserror::Error as ThisError;

pub type BridgeBasicResult<T> = Result<T, BridgeBasicError>;

#[derive(ThisError, Debug)]
pub enum BridgeBasicError {
    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Custom error: {0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}
