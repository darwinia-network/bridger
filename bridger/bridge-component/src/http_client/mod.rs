pub use self::config::*;

use bridge_standard::component::BridgeComponent;
use std::time::Duration;

mod config;

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
