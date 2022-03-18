use gql_client::GraphQLError;
use thiserror::Error as ThisError;

/// Shadow component result
pub type ShadowComponentReuslt<T> = Result<T, ShadowComponentError>;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum ShadowComponentError {
    #[error("Internal server error: {0}")]
    InternalServer(String),
    #[error("Failed to send query request to subquery: {0}")]
    GraphQL(String),
    #[error("MMR: {0}")]
    MMR(String),
    #[error("Ethereum: {0}")]
    Ethereum(String),
}

impl From<GraphQLError> for ShadowComponentError {
    fn from(error: GraphQLError) -> Self {
        Self::GraphQL(format!("{:?}", error))
    }
}
