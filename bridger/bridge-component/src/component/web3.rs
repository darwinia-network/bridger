use bridge_config::config::component::Web3Config;
use bridge_standard::bridge::component::BridgeComponent;
use bridge_standard::error::StandardError;
use web3::transports::Http;
use web3::Web3;

pub struct Web3Component {
    config: Web3Config,
}

impl Web3Component {
    pub fn new(config: Web3Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl BridgeComponent<Web3Config, Web3<Http>> for Web3Component {
    async fn component(&self) -> anyhow::Result<Web3<Http>> {
        Ok(Web3::new(Http::new(&self.config.endpoint).map_err(
            |e| StandardError::NewHttpError(self.config.endpoint.clone(), e.to_string()),
        )?))
    }

    fn config(&self) -> &Web3Config {
        &self.config
    }
}
