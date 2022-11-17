use bridge_e2e_traits::error::E2EClientError;
use thiserror::Error as ThisError;

pub type RelayResult<T> = Result<T, RelayError>;

#[derive(ThisError, Debug)]
pub enum RelayError {
    #[error(transparent)]
    Client(#[from] E2EClientError),
    #[error("Bytes: {0}")]
    Bytes(String),
    #[error("Seed: {0}")]
    Seed(String),
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
