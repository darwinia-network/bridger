use gql_client::GraphQLError;
use thiserror::Error as ThisError;

pub type SubqueryComponentResult<T> = Result<T, SubqueryComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum SubqueryComponentError {
    #[error("Failed to send query request to subquery: {0}")]
    GraphQL(String),

    #[cfg(feature = "array-bytes")]
    #[error("Bytes error: {0}")]
    Bytes(String),
}

impl From<GraphQLError> for SubqueryComponentError {
    fn from(error: GraphQLError) -> Self {
        Self::GraphQL(format!("{:?}", error))
    }
}

#[cfg(feature = "array-bytes")]
impl From<array_bytes::Error> for SubqueryComponentError {
    fn from(error: array_bytes::Error) -> Self {
        Self::Bytes(format!("{:?}", error))
    }
}
