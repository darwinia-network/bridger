use crate::config::ThegraphConfig;
use crate::error::ThegraphComponentReuslt;
use crate::thegraph::Thegraph;
use crate::types::LikethChain;

/// thGraph component
pub struct ThegraphComponent;

impl ThegraphComponent {
    /// Get theGraph instance
    pub fn component(
        config: ThegraphConfig,
        chain: LikethChain,
    ) -> ThegraphComponentReuslt<Thegraph> {
        let client = gql_client::Client::new(config.endpoint);
        Ok(Thegraph::new(client, chain))
    }
}
