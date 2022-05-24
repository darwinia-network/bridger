#![allow(missing_docs)]

use support_toolkit::error::TkError;
use thiserror::Error as ThisError;

pub type ClientResult<T> = Result<T, ClientError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error(transparent)]
    SubxtBasicError(subxt::BasicError),

    #[error("Subxt Runtime Error: {0}")]
    SubxtRuntime(String),

    #[error("Please reconnect to rpc server")]
    ClientRestartNeed,

    #[error("Wrong seed: {0}")]
    Seed(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Tk(#[from] TkError),
}

impl ClientError {
    /// Is restart need error
    pub fn is_restart_need(&self) -> bool {
        matches!(self, Self::ClientRestartNeed)
    }
}

impl From<subxt::BasicError> for ClientError {
    fn from(error: subxt::BasicError) -> Self {
        if let subxt::BasicError::Rpc(_) = &error {
            return Self::ClientRestartNeed;
        }
        Self::SubxtBasicError(error)
    }
}
