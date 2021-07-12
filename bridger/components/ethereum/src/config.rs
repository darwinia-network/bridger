use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

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

    fn template() -> Self {
        Self {
            rpc: vec!["https://mainnet.infura.io/v3/<api_key>".to_string()],
            atom: 0,
        }
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

    fn template() -> Self {
        Self {
            endpoint: "https://mainnet.infura.io/v3/<api_key>".to_string(),
        }
    }
}
