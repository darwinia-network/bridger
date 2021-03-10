use jsonrpsee::client::RequestError;
use thiserror::Error as ThisError;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum Error {
	/// Io error.
	#[error("Io error: {0}")]
	Io(#[from] std::io::Error),
	/// Rpc error.
	#[error("Rpc error: {0}")]
	Rpc(#[from] RequestError),
	/// Serde serialization error
	#[error("Serde json error: {0}")]
	Serialization(#[from] serde_json::error::Error),
	/// Other error.
	#[error("Other error: {0}")]
	Other(String),
	#[error("No signer seed set for authority, please check your config.toml")]
	NoAuthoritySignerSeed,
	#[error("`bytes2hex` - FAILED: {0}")]
	Bytes2Hex(String),
	#[error("`hex2bytes` - FAILED: {0}")]
	Hex2Bytes(String),
}

/// Error enum.
#[derive(ThisError, Debug)]
pub enum DarwiniaError {
	#[error("{0}")]
	Bridger(String),
}

pub type Result<T> = anyhow::Result<T>;
