use serde::{Deserialize, Serialize};
use web3::transports::Http;
use web3::Web3;

/// Web3 provider, get web3 instance
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web3Config {
    /// Then endpoint for web3
    pub endpoint: String,
}

impl Default for Web3Config {
    fn default() -> Self {
        Self {
            endpoint: "https://mainnet.infura.io/v3/<api_key>".to_string(),
        }
    }
}

/// Web3 component
pub struct Web3Component;

impl Web3Component {
    /// Get web3 instance
    pub fn component(config: Web3Config) -> color_eyre::Result<Web3<Http>> {
        Ok(Web3::new(Http::new(&config.endpoint)?))
    }
}
