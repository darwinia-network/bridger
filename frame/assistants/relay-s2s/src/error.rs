use abstract_bridge_s2s::error::S2SClientError;
use sp_runtime::codec;
use subquery_s2s::SubqueryComponentError;
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

impl From<subquery_parachain::SubqueryComponentError> for RelayError {
    fn from(error: subquery_parachain::SubqueryComponentError) -> Self {
        match error {
            subquery_parachain::SubqueryComponentError::GraphQL(message) => {
                RelayError::Subquery(SubqueryComponentError::GraphQL(message))
            }
        }
    }
}
