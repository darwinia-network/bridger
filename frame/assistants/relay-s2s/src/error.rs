use bridge_s2s_traits::error::S2SClientError;
use sp_runtime::codec;
use subquery::SubqueryComponentError;
use thiserror::Error as ThisError;

pub type RelayResult<T> = Result<T, RelayError>;

#[derive(ThisError, Debug)]
pub enum RelayError {
    #[error(transparent)]
    Subquery(#[from] SubqueryComponentError),
    #[error(transparent)]
    Client(#[from] S2SClientError),
    #[error(transparent)]
    Codec(#[from] codec::Error),
    #[error("Custom: {0}")]
    Custom(String),
}
