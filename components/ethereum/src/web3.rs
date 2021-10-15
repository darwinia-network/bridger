use web3::transports::Http;
use web3::Web3;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::{BridgeResult, StandardError};

use crate::config::Web3Config;

#[derive(Clone, Debug, Default)]
pub struct Web3Component {
    config: Web3Config,
}

impl Web3Component {
    pub fn new(config: Web3Config) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<Web3Config, Web3<Http>> for Web3Component {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: Web3Config = Config::restore_with_namespace(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<Web3<Http>> {
        Ok(Web3::new(Http::new(&self.config.endpoint).map_err(
            |e| StandardError::NewHttpError(self.config.endpoint.clone(), e.to_string()),
        )?))
    }

    fn config(&self) -> &Web3Config {
        &self.config
    }
}
