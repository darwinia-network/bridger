//! Bridge Result
#![allow(missing_docs)]
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "rpc")]
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "rpc")]
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

/// Sup Result
pub type Result<T> = std::result::Result<T, Error>;
