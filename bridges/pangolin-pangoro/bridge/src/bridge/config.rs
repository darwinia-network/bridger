use serde::{Deserialize, Serialize};

use crate::types::HexLaneId;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    /// Pangolin chain
    pub pangolin: ChainInfoConfig,
    /// Panogro chain
    pub pangoro: ChainInfoConfig,
    /// Relay config
    pub relay: RelayConfig,
}

/// Chain info
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainInfoConfig {
    /// Endpoint
    pub endpoint: String,
    /// Signer
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
}

impl From<ChainInfoConfig> for client_pangolin::config::ClientConfig {
    fn from(config: ChainInfoConfig) -> Self {
        client_pangolin::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.signer,
            relayer_real_account: None,
        }
    }
}

impl From<ChainInfoConfig> for client_pangoro::config::ClientConfig {
    fn from(config: ChainInfoConfig) -> Self {
        client_pangoro::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.signer,
            relayer_real_account: None,
        }
    }
}
