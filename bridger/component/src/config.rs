use std::path::PathBuf;

use bridge_traits::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthereumRpcConfig {
    /// Rpc host
    pub rpc: Vec<String>,
    /// Counter
    pub atom: usize,
}

impl BridgeConfig for EthereumRpcConfig {
    fn marker() -> &'static str {
        "component-ethereum_rpc"
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub endpoint: String,
}

impl BridgeConfig for ShadowConfig {
    fn marker() -> &'static str {
        "component-shadow"
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BeeConfig {
    pub endpoint: String,
    pub strict: bool,
}

impl BridgeConfig for BeeConfig {
    fn marker() -> &'static str {
        "component-bee"
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
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Web3Config {
    pub endpoint: String,
}

impl BridgeConfig for Web3Config {
    fn marker() -> &'static str {
        "component-web3"
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MicrokvConfig {
    pub base_path: PathBuf,
    pub db_name: Option<String>,
    pub auto_commit: bool,
}

impl BridgeConfig for MicrokvConfig {
    fn marker() -> &'static str {
        "component-microkv"
    }
}
