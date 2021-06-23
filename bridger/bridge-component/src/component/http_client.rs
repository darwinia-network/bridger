use bridge_config::component::HttpClientConfig;
use bridge_standard::bridge::component::BridgeComponent;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct HttpClientComponent {
    config: HttpClientConfig,
}

impl HttpClientComponent {
    pub fn new(config: HttpClientConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl BridgeComponent<HttpClientConfig, reqwest::Client> for HttpClientComponent {
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
