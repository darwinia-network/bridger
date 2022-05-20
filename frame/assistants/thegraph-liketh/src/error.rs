use gql_client::GraphQLError;
use thiserror::Error as ThisError;

/// TheGraphLiketh component result
pub type TheGraphLikethComponentReuslt<T> = Result<T, TheGraphLikethComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum TheGraphLikethComponentError {
    #[error("Failed to send query request to subquery: {0}")]
    GraphQL(String),

    #[error("Failed to query transaction: {0}")]
    UnknownResponse(String),
}

impl From<GraphQLError> for TheGraphLikethComponentError {
    fn from(error: GraphQLError) -> Self {
        Self::GraphQL(format!("{:?}", error))
    }
}
