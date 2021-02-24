use thiserror::Error as ThisError;

use jsonrpsee::client::RequestError;

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
}

/// Error enum.
#[derive(ThisError, Debug)]
pub enum DarwiniaError {
	#[error("{0}")]
	Bridger(String),
}

pub type Result<T> = anyhow::Result<T>;
