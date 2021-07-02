use serde::{Deserialize, Serialize};

use bridge_component::config::HttpClientConfig;
use bridge_standard::bridge::config::Config;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateLinkedConfig {
    pub http_client: HttpClientConfig,
}

impl TemplateLinkedConfig {
    pub fn store<S: AsRef<str>>(&self, sand_name: S) -> anyhow::Result<()> {
        let sand_name = sand_name.as_ref();
        Config::store(sand_name, self.http_client.clone())?;
        Ok(())
    }
}
