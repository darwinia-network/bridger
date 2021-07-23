use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Rpc host
    pub endpoint: Vec<String>,

    /// Relayer
    pub relayer_private_key: Option<String>,
    pub relayer_beneficiary_darwinia_account: Option<String>,

    /// ring & kton address
    pub subscribe_ring_address: String,
    pub subscribe_kton_address: String,

    /// Subscribe contract with topics
    pub subscribe_bank_address: String,
    pub subscribe_bank_topics: Vec<String>,
    pub subscribe_relay_address: String,
    pub subscribe_relay_topics: Vec<String>,
    pub subscribe_issuing_address: String,
    pub subscribe_issuing_topics: Vec<String>,

    /// Counter
    pub atom: usize,
}

impl BridgeConfig for EthereumConfig {
    fn marker() -> &'static str {
        "component-ethereum_rpc"
    }

    fn template() -> Self {
        Self {
            endpoint: vec!["https://mainnet.infura.io/v3/<api_key>".to_string()],
            relayer_private_key: Some("0x...".to_string()),
            relayer_beneficiary_darwinia_account: Some("0x...".to_string()),
            subscribe_ring_address: "0x...".to_string(),
            subscribe_kton_address: "0x...".to_string(),
            subscribe_bank_address: "0x...".to_string(),
            subscribe_bank_topics: vec!["0x...".to_string()],
            subscribe_relay_address: "0x...".to_string(),
            subscribe_relay_topics: vec!["0x...".to_string()],
            subscribe_issuing_address: "0x...".to_string(),
            subscribe_issuing_topics: vec!["0x...".to_string()],
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
