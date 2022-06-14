use serde::{Deserialize, Serialize};
use subquery_s2s::SubqueryConfig;

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
    /// Index config
    pub index: IndexConfig,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub pangolin: SubqueryConfig,
    pub pangoro: SubqueryConfig,
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

impl ChainInfoConfig {
    pub fn to_pangolin_client_config(
        &self,
    ) -> color_eyre::Result<client_pangolin::config::ClientConfig> {
        Ok(client_pangolin::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        })
    }

    pub fn to_pangoro_client_config(
        &self,
    ) -> color_eyre::Result<client_pangoro::config::ClientConfig> {
        Ok(client_pangoro::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        })
    }
}
