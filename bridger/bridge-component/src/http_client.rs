use bridge_standard::component::BridgeComponent;
use bridge_standard::config::BridgeConfig;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct HttpClientConfig {
    pub timeout: u64,
}

impl BridgeConfig for HttpClientConfig {}

#[derive(Clone, Debug, Default)]
pub struct HttpClientComponent {
    config: HttpClientConfig,
}

impl HttpClientComponent {
    pub fn new(config: HttpClientConfig) -> anyhow::Result<Self> {
        Ok(Self { config })
    }
}

impl BridgeComponent<HttpClientConfig, reqwest::Client> for HttpClientComponent {
    fn component(&self) -> anyhow::Result<reqwest::Client> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.timeout))
            .build()?;
        Ok(client)
    }

    fn config(&self) -> &HttpClientConfig {
        &self.config
    }
}
