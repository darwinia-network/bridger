use abstract_client_s2s::error::S2SClientError;
use subquery_s2s::SubqueryComponentError;
use thiserror::Error as ThisError;

pub type RelayResult<T> = Result<T, RelayError>;

#[derive(ThisError, Debug)]
pub enum RelayError {
    #[error(transparent)]
    Subquery(#[from] SubqueryComponentError),
    #[error(transparent)]
    Client(#[from] S2SClientError),
    #[error("Custom: {0}")]
    Custom(String),
}
