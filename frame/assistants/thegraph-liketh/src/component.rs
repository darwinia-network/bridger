use crate::config::TheGraphLikeEthConfig;
use crate::error::TheGraphLikethComponentReuslt;
use crate::graph::TheGraphLikeEth;
use crate::types::LikethChain;

/// thGraph component
pub struct TheGraphLikeEthComponent;

impl TheGraphLikeEthComponent {
    /// Get theGraph instance
    pub fn component(
        config: TheGraphLikeEthConfig,
        chain: LikethChain,
    ) -> TheGraphLikethComponentReuslt<TheGraphLikeEth> {
        let client = gql_client::Client::new(config.endpoint);
        Ok(TheGraphLikeEth::new(client, chain))
    }
}
