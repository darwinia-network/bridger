use gql_client::Client;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::TheGraphLikeEthConfig;
use crate::graph::TheGraphLikeEth;

pub mod config;
pub mod graph;
pub mod types;

#[derive(Clone, Debug, Default)]
pub struct TheGraphLikeEthComponent {
    config: TheGraphLikeEthConfig,
}

impl TheGraphLikeEthComponent {
    pub fn new(config: TheGraphLikeEthConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<TheGraphLikeEthConfig, TheGraphLikeEth> for TheGraphLikeEthComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
    where
        Self: Sized,
    {
        let config: TheGraphLikeEthConfig = Config::restore_with_namespace(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<TheGraphLikeEth> {
        let client = Client::new(&self.config.endpoint[..]);
        Ok(TheGraphLikeEth::new(client))
    }

    fn config(&self) -> &TheGraphLikeEthConfig {
        &self.config
    }
}
