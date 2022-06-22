#![allow(missing_docs)]

use support_toolkit::error::TkError;
use thiserror::Error as ThisError;

pub type ClientResult<T> = Result<T, ClientError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error(transparent)]
    SubxtBasicError(subxt::BasicError),

    #[error("Please reconnect to rpc server")]
    ClientRestartNeed,

    #[error(transparent)]
    Codec(#[from] codec::Error),

    #[error("No header hash in EthereumReceiptProofOfThing")]
    NoHeaderHashInEthereumReceiptProofOfThing,

    #[error("Wrong seed: {0}")]
    Seed(String),

    #[error("Bytes error: {0}")]
    Bytes(String),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    // #[error("Rpc error: {0}")]
    // Rpc(#[from] jsonrpsee_types::error::Error),
    #[error("Serde json error: {0}")]
    Serialization(#[from] serde_json::error::Error),

    #[cfg(feature = "ethlike-v1")]
    #[error("Failed to build SecretKey from authority's private key")]
    FailedToBuildSecretKey(#[from] secp256k1::Error),

    #[cfg(feature = "ethlike-v1")]
    #[error("Failed to connect ethereum rpc http endpoint")]
    CannotConnectToWeb3(#[from] web3::Error),

    #[error("No signer seed set for authority, please check your config.toml")]
    NoAuthoritySignerSeed,

    #[error("`bytes2hex` - FAILED: {0}")]
    Bytes2Hex(String),

    #[error("`hex2bytes` - FAILED: {0}")]
    Hex2Bytes(String),

    #[error("Wrong mmr_root({0}) in Darwinia header({1})")]
    WrongMmrRootInDarwiniaHeader(String, u32),

    #[error("No mmr_root in Darwinia header({0})")]
    NoMmrRootInDarwiniaHeader(u32),

    #[error("Failed to fetch Darwinia header({0})")]
    FailedToFetchDarwiniaHeader(u32),

    #[error("No storage data found by {0} {1}")]
    NoStorageDataFound(String, String),

    #[error("Not technical committee member")]
    NotTechnicalCommitteeMember,

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

impl From<subxt::rpc::RpcError> for ClientError {
    fn from(error: subxt::rpc::RpcError) -> Self {
        Self::SubxtBasicError(subxt::BasicError::Rpc(error))
    }
}

impl From<array_bytes::Error> for ClientError {
    fn from(error: array_bytes::Error) -> Self {
        Self::Bytes(format!("{:?}", error))
    }
}

#[cfg(feature = "s2s")]
impl From<ClientError> for abstract_bridge_s2s::error::S2SClientError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::SubxtBasicError(e) => Self::RPC(format!("{:?}", e)),
            ClientError::ClientRestartNeed => Self::RPC(format!("Client restart need")),
            _ => Self::Custom(format!("{:?}", error)),
        }
    }
}
