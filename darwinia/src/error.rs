#![allow(missing_docs)]

use jsonrpsee::client::RequestError;
use thiserror::Error as ThisError;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Io error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Rpc error: {0}")]
	Rpc(#[from] RequestError),

	#[error("Serde json error: {0}")]
	Serialization(#[from] serde_json::error::Error),

	#[error("Failed to connect darwinia rpc ws endpoint")]
	CannotConnectToDarwinia(#[from] jsonrpsee::transport::ws::WsNewDnsError),

	#[error("Failed to build SecretKey from authority's private key")]
	FailedToBuildSecretKey(#[from] secp256k1::Error),

	#[error("Failed to connect ethereum rpc http endpoint")]
	CannotConnectToWeb3(#[from] web3::Error),

	#[error(transparent)]
	SubxtError(#[from] substrate_subxt::Error),

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

	#[error("No header hash in EthereumReceiptProofOfThing")]
	NoHeaderHashInEthereumReceiptProofOfThing,

	#[error("Other error: {0}")]
	Other(String),
}

pub type Result<T> = anyhow::Result<T, Error>;
