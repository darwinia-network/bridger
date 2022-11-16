use relay_s2s::error::RelayError;
use thiserror::Error as ThisError;

pub type BinS2SResult<T> = Result<T, BinS2SError>;

#[derive(ThisError, Debug)]
pub enum BinS2SError {
    #[error(transparent)]
    Relay(#[from] RelayError),
    #[error("Lifeline: {0}")]
    Lifeline(String),
    #[error("Client: {0}")]
    Client(String),
    #[error("Custom: {0}")]
    Custom(String),
}
