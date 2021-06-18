use bridge_standard::component::BridgeComponent;
use bridge_standard::config::BridgeConfig;
use bridge_standard::error::StandardError;
use web3::transports::Http;
use web3::Web3;

pub struct Web3Config {
    pub endpoint: String,
}

impl BridgeConfig for Web3Config {}

pub struct Web3Component {
    config: Web3Config,
}

impl Web3Component {
    pub fn new(config: Web3Config) -> anyhow::Result<Self> {
        Ok(Self { config })
    }
}

impl BridgeComponent<Web3Config, Web3<Http>> for Web3Component {
    fn component(&self) -> anyhow::Result<Web3<Http>> {
        Ok(Web3::new(Http::new(&self.config.endpoint).map_err(
            |e| StandardError::NewHttpError(self.config.endpoint.clone(), e.to_string()),
        )?))
    }

    fn config(&self) -> &Web3Config {
        &self.config
    }
}
