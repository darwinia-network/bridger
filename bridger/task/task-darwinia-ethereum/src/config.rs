use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::BridgeConfig;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubstrateEthereumConfig {
    /// ethereum scan service polling interval in seconds
    pub interval_ethereum: u64,
    /// relay service polling interval in seconds
    pub interval_relay: u64,
    /// redeem service polling interval in seconds
    pub interval_redeem: u64,
    /// guard service polling interval in seconds
    pub interval_guard: u64,
}

impl BridgeConfig for SubstrateEthereumConfig {
    fn marker() -> &'static str {
        "service-substrate-ethereum"
    }
}
