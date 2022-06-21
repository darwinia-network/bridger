use thiserror::Error as ThisError;

pub type S2SClientResult<T> = Result<T, S2SClientError>;

#[derive(ThisError, Debug)]
pub enum S2SClientError {
    #[error("RPC: {0}")]
    RPC(String),
    #[error(transparent)]
    Codec(#[from] codec::Error),
    #[error("Custom: {0}")]
    Custom(String),
}
