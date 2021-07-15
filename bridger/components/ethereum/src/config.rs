use bridge_traits::bridge::config::BridgeConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Rpc host
    pub endpoint: Vec<String>,

    /// Relayer
    pub relayer_private_key: Option<String>,
    pub relayer_beneficiary_darwinia_account: Option<String>,
    pub relayer_relay_contract_address: Option<String>,

    /// Subscribe topics
    pub subscribe_ring_address: String,
    pub subscribe_ring_topics: Vec<String>,
    pub subscribe_kton_address: String,
    pub subscribe_kton_topics: Vec<String>,
    pub subscribe_bank_address: String,
    pub subscribe_bank_topics: Vec<String>,
    pub subscribe_relay_address: String,
    pub subscribe_relay_topics: Vec<String>,
    pub subscribe_backing_address: String,
    pub subscribe_backing_topics: Vec<String>,

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
            relayer_private_key: None,
            relayer_beneficiary_darwinia_account: None,
            relayer_relay_contract_address: None,
            subscribe_ring_address: "".to_string(),
            subscribe_ring_topics: vec![],
            subscribe_kton_address: "".to_string(),
            subscribe_kton_topics: vec![],
            subscribe_bank_address: "".to_string(),
            subscribe_bank_topics: vec![],
            subscribe_relay_address: "".to_string(),
            subscribe_relay_topics: vec![],
            subscribe_backing_address: "".to_string(),
            subscribe_backing_topics: vec![],
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
