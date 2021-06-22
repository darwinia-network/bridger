use bridge_standard::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default)]
pub struct EthereumRpcConfig {
    /// Rpc host
    pub rpc: Vec<String>,
    /// Counter
    pub atom: usize,
}

impl BridgeConfig for EthereumRpcConfig {}

#[derive(Clone, Debug, Default)]
pub struct ShadowConfig {
    pub endpoint: String,
}

impl BridgeConfig for ShadowConfig {}

#[derive(Clone, Debug, Default)]
pub struct BeeConfig {
    pub endpoint: String,
    pub strict: bool,
}

impl BridgeConfig for BeeConfig {}

#[derive(Clone, Debug, Default)]
pub struct HttpClientConfig {
    pub timeout: u64,
}

impl BridgeConfig for HttpClientConfig {}

#[derive(Clone, Debug, Default)]
pub struct Web3Config {
    pub endpoint: String,
}

impl BridgeConfig for Web3Config {}
