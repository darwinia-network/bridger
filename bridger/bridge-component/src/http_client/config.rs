use bridge_standard::config::BridgeConfig;

#[derive(Clone, Debug, Default)]
pub struct HttpClientConfig {
	pub timeout: u64,
}

impl BridgeConfig for HttpClientConfig {}
