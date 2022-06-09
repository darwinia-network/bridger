#![allow(missing_docs)]

use jsonrpsee::core::error::Error as RpcError;
use support_toolkit::error::TkError;
use thiserror::Error as ThisError;

pub type ClientResult<T> = Result<T, ClientError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error(transparent)]
    SubxtBasicError(subxt::BasicError),

    #[error(transparent)]
    RpcBasicError(RpcError),

    #[error("Please reconnect to rpc server")]
    ClientRestartNeed,

    #[error("Wrong seed: {0}")]
    Seed(String),

    #[error(transparent)]
    Tk(#[from] TkError),

    #[error("Other error: {0}")]
    Other(String),
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

impl From<RpcError> for ClientError {
    fn from(error: RpcError) -> Self {
        if let RpcError::RestartNeeded(_) = &error {
            return Self::ClientRestartNeed;
        }
        Self::RpcBasicError(error)
    }
}
