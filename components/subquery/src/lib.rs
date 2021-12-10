use gql_client::Client;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

use crate::config::SubqueryConfig;
use crate::subquery::Subquery;

pub mod config;
pub mod subquery;
pub mod types;

#[derive(Clone, Debug, Default)]
pub struct SubqueryComponent {
    config: SubqueryConfig,
}

impl SubqueryComponent {
    pub fn new(config: SubqueryConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<SubqueryConfig, Subquery> for SubqueryComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self>
    where
        Self: Sized,
    {
        let config: SubqueryConfig = Config::restore_with_namespace_unwrap(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<Subquery> {
        let client = Client::new(&self.config.endpoint[..]);
        Ok(Subquery::new(client))
    }

    fn config(&self) -> &SubqueryConfig {
        &self.config
    }
}
