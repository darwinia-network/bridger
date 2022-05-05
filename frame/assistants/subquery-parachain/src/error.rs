use gql_client::GraphQLError;
use thiserror::Error as ThisError;

pub type SubqueryComponentResult<T> = Result<T, SubqueryComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum SubqueryComponentError {
    #[error("Failed to send query request to subquery: {0}")]
    GraphQL(String),
}

impl From<GraphQLError> for SubqueryComponentError {
    fn from(error: GraphQLError) -> Self {
        Self::GraphQL(format!("{:?}", error))
    }
}
