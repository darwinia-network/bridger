use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Relayer
    pub relayer_private_key: Option<String>,
    pub relayer_beneficiary_darwinia_account: Option<String>,

    pub subscribe_relay_address: String,
}

impl BridgeConfig for EthereumConfig {
    fn marker() -> &'static str {
        "component-ethereum_rpc"
    }

    fn template() -> Self {
        Self {
            relayer_private_key: Some("0x...".to_string()),
            relayer_beneficiary_darwinia_account: Some("0x...".to_string()),
            subscribe_relay_address: "0x...".to_string(),
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
