#![allow(missing_docs)]

use thiserror::Error as ThisError;

pub type ComponentResult<T> = core::result::Result<T, ComponentError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum ComponentError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("Fail to get ethereum header of block {1}: {0}")]
    FailToGetEthereumHeader(String, u64),
}
