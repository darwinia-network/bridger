use crate::config::TheGraphLikeEthConfig;
use crate::graph::TheGraphLikeEth;

/// thGraph component
pub struct TheGraphLikeEthComponent;

impl TheGraphLikeEthComponent {
    /// Get theGraph instance
    pub fn component(config: TheGraphLikeEthConfig) -> color_eyre::Result<TheGraphLikeEth> {
        let client = gql_client::Client::new(config.endpoint);
        Ok(TheGraphLikeEth::new(client))
    }
}
