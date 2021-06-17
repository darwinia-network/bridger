#![allow(missing_docs)]

use thiserror::Error as ThisError;

pub type BeeResult<T> = core::result::Result<T, BridgeError>;
/// Error enum.
#[derive(ThisError, Debug)]
pub enum BridgeError {
	#[error("Io error: {0}")]
	Io(#[from] std::io::Error),

	#[error("Not support this feature: {0}")]
	NotSupport(String),

	#[error("Other error: {0}")]
	Other(String),

	#[error("Custom error: {0}")]
	Custom(Box<dyn std::error::Error + Send + Sync>),
}
