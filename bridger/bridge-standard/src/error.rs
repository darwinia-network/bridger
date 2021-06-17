#![allow(missing_docs)]

use thiserror::Error as ThisError;

pub type BridgeResult<T> = core::result::Result<T, StandardError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum StandardError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not support this feature: {0}")]
    NotSupport(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Custom error: {0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}
