use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error(transparent)]
	Web3Error(#[from] web3::Error),
	#[error("Other error: {0}")]
	Other(String),
}

pub type Result<T> = anyhow::Result<T, anyhow::Error>;

