use gql_client::GraphQLError;
use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum TheGraphLikethComponentError {
    #[error("Failed to send query request to subquery: {0}")]
    GraphQL(String),

    #[error("Failed to query transaction page. query: {0}, vars: [from: {1}, limit: {2}]")]
    UnknownResponse(String, u64, u32),
}

impl From<GraphQLError> for TheGraphLikethComponentError {
    fn from(error: GraphQLError) -> Self {
        Self::GraphQL(format!("{:?}", error))
    }
}
