use thiserror::Error as ThisError;

/// Bridge ethereum error
#[derive(ThisError, Debug)]
pub enum ConvertTypeError {
    #[error("Other error: {0}")]
    Other(String),

    #[error("Array bytes error: {0}")]
    ArrayBytes(String),
}

impl From<array_bytes::Error> for ConvertTypeError {
    fn from(error: array_bytes::Error) -> Self {
        Self::ArrayBytes(format!("{:?}", error))
    }
}
