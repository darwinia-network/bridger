use anyhow::Error as AnyError;
use thiserror::Error as ThisError;
use web3::contract::Error as ContractError;
use web3::ethabi::Error as EthError;
use web3::Error as Web3Error;

pub type BridgeContractResult<T> = Result<T, BridgeContractError>;

#[derive(ThisError, Debug)]
pub enum BridgeContractError {
    #[error("Custom: {0}")]
    Custom(String),
    #[error(transparent)]
    Web3(#[from] Web3Error),
    #[error(transparent)]
    Eth(#[from] EthError),
    #[error(transparent)]
    Contract(#[from] ContractError),
    #[error(transparent)]
    Other(#[from] AnyError),
}
