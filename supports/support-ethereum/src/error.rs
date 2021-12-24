use thiserror::Error as ThisError;

/// Bridge ethereum error
pub enum BridgeEthereumError {
    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Array bytes error: {0}")]
    ArrayBytes(#[from] array_bytes::Error),

    #[error("Custom error: {0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),
}
