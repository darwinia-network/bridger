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

impl From<subxt::Error> for S2SClientError {
    fn from(error: subxt::Error) -> Self {
        Self::RPC(format!("{error:?}"))
    }
}

impl From<subxt::error::RpcError> for S2SClientError {
    fn from(error: subxt::error::RpcError) -> Self {
        Self::RPC(format!("{error:?}"))
    }
}

#[cfg(feature = "array-bytes")]
impl From<array_bytes::Error> for S2SClientError {
    fn from(error: array_bytes::Error) -> Self {
        Self::Custom(format!("[bytes] {:?}", error))
    }
}
