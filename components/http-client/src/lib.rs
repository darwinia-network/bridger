use std::time::Duration;

use serde::{Deserialize, Serialize};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::{BridgeConfig, Config};
use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::BridgeResult;

#[derive(Clone, Debug, Default)]
pub struct HttpClientComponent {
    config: HttpClientConfig,
}

impl HttpClientComponent {
    pub fn new(config: HttpClientConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl BridgeComponent<HttpClientConfig, reqwest::Client> for HttpClientComponent {
    fn restore_with_namespace<T: BridgeSand>(namespace: String) -> BridgeResult<Self> {
        let config: HttpClientConfig = Config::restore_with_namespace(T::NAME, namespace)?;
        Ok(Self::new(config))
    }

    async fn component(&self) -> anyhow::Result<reqwest::Client> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.timeout))
            .build()?;
        Ok(client)
    }

    fn config(&self) -> &HttpClientConfig {
        &self.config
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HttpClientConfig {
    pub timeout: u64,
}

impl BridgeConfig for HttpClientConfig {
    fn marker() -> &'static str {
        "component-http_client"
    }

    fn template() -> Self {
        Self { timeout: 3000 }
    }
}
