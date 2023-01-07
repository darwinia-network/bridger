use thiserror::Error as ThisError;

pub type AbstractFeemarketResult<T> = Result<T, AbstractFeemarketError>;

#[derive(ThisError, Debug)]
pub enum AbstractFeemarketError {
    #[error("RPC: {0}")]
    RPC(String),
    #[error(transparent)]
    Codec(#[from] codec::Error),
    #[error("Custom: {0}")]
    Custom(String),
}

#[cfg(feature = "subxt")]
impl From<subxt::error::Error> for AbstractFeemarketError {
    fn from(error: subxt::error::Error) -> Self {
        Self::RPC(format!("{:?}", error))
    }
}

#[cfg(feature = "subxt")]
impl From<subxt::error::RpcError> for AbstractFeemarketError {
    fn from(error: subxt::error::RpcError) -> Self {
        Self::RPC(format!("{:?}", error))
    }
}
