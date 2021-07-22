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

    #[error("The last redeemed block number is not set")]
    LastRedeemedFileNotExists,

    #[error("No signer seed set for authority, please check your config.toml")]
    NoAuthoritySignerSeed,

    #[error("RuntimeUpdated")]
    RuntimeUpdated,

    #[error("ShadowInternalServerError: {0}")]
    ShadowInternalServerError(String),

    #[error("`bytes2hex` - FAILED: {0}")]
    Bytes2Hex(String),

    #[error("`hex2bytes` - FAILED: {0}")]
    Hex2Bytes(String),

    #[error("New http with URI {0} error: {1}")]
    NewHttpError(String, String),

    #[error("Api error: {0}")]
    Api(String),

    #[error("Cli error: {0}")]
    Cli(String),
}
