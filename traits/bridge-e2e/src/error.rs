use client_contracts::error::BridgeContractError;
use thiserror::Error as ThisError;

pub type E2EClientResult<T> = Result<T, E2EClientError>;

#[derive(ThisError, Debug)]
pub enum E2EClientError {
    #[error("RPC: {0}")]
    RPC(String),
    #[error(transparent)]
    ContractError(#[from] BridgeContractError),
    #[error(transparent)]
    Web3Error(#[from] web3::Error),
    #[error(transparent)]
    Web3AbiError(#[from] web3::ethabi::Error),
    #[error("Custom: {0}")]
    Custom(String),
}

impl From<subxt::Error> for E2EClientError {
    fn from(error: subxt::Error) -> Self {
        Self::RPC(format!("{error:?}"))
    }
}

impl From<subxt::error::RpcError> for E2EClientError {
    fn from(error: subxt::error::RpcError) -> Self {
        Self::RPC(format!("{error:?}"))
    }
}
