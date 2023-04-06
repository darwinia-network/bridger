use bridge_e2e_traits::error::E2EClientError;
use client_beacon::error::BeaconApiError;
use client_contracts::error::BridgeContractError;
use subquery::SubqueryComponentError;
use support_etherscan::Error as SupportEtherscanError;
use thiserror::Error as ThisError;
use web3::Error as Web3Error;
use types::BeaconStateError;

pub type RelayResult<T> = Result<T, RelayError>;

#[derive(ThisError, Debug)]
pub enum RelayError {
    #[error(transparent)]
    Client(#[from] E2EClientError),
    #[error("Bytes: {0}")]
    Bytes(String),
    #[error("Seed: {0}")]
    Seed(String),
    #[error(transparent)]
    Web3Error(#[from] Web3Error),
    #[error(transparent)]
    ContractError(#[from] BridgeContractError),
    #[error(transparent)]
    BeaconApiError(#[from] BeaconApiError),
    #[error(transparent)]
    EtherscanError(#[from] SupportEtherscanError),
    #[error(transparent)]
    SubqueryError(#[from] SubqueryComponentError),
    #[error("Custom: {0}")]
    Custom(String),
}

impl From<array_bytes::Error> for RelayError {
    fn from(e: array_bytes::Error) -> Self {
        Self::Bytes(format!("{:?}", e))
    }
}

impl From<secp256k1::Error> for RelayError {
    fn from(e: secp256k1::Error) -> Self {
        Self::Seed(format!("Wrong secret key: {:?}", e))
    }
}

impl From<()> for RelayError {
    fn from(_: ()) -> Self {
        Self::Custom("Incorrect beacon api version.".into())
    }
}

impl From<BeaconStateError> for RelayError {
    fn from(value: BeaconStateError) -> Self {
        Self::Custom(format!("{:?}", value))
    }
}