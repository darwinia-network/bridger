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

#[cfg(feature = "subxt")]
impl From<subxt::BasicError> for S2SClientError {
    fn from(error: subxt::BasicError) -> Self {
        Self::RPC(format!("{:?}", error))
    }
}

#[cfg(feature = "subxt")]
impl From<subxt::rpc::RpcError> for S2SClientError {
    fn from(error: subxt::rpc::RpcError) -> Self {
        Self::RPC(format!("{:?}", error))
    }
}

#[cfg(feature = "array-bytes")]
impl From<array_bytes::Error> for S2SClientError {
    fn from(error: array_bytes::Error) -> Self {
        Self::Custom(format!("[bytes] {:?}", error))
    }
}
