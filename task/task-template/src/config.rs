use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::{BridgeConfig, Config};
use component_http_client::HttpClientConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateTaskConfig {
    pub http_client: HttpClientConfig,
}

impl TemplateTaskConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        let sand_name = sand_name.as_ref();
        Config::store(sand_name, self.http_client.clone())?;
        Ok(())
    }
    pub fn template() -> Self {
        Self {
            http_client: HttpClientConfig::template(),
        }
    }
}
