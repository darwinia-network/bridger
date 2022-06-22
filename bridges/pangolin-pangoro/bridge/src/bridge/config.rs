use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use serde::{Deserialize, Serialize};
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent, SubqueryConfig};

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
    pub async fn to_pangolin_client(&self) -> color_eyre::Result<PangolinClient> {
        let config = client_pangolin::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangolinClientComponent::component(config).await?)
    }

    pub async fn to_pangoro_client(&self) -> color_eyre::Result<PangoroClient> {
        let config = client_pangoro::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangoroClientComponent::component(config).await?)
    }
}

impl IndexConfig {
    pub fn to_pangolin_subquery(&self) -> color_eyre::Result<Subquery> {
        Ok(SubqueryComponent::component(
            self.pangolin.clone(),
            BridgeName::PangolinPangoro,
        ))
    }

    pub fn to_pangoro_subquery(&self) -> color_eyre::Result<Subquery> {
        Ok(SubqueryComponent::component(
            self.pangoro.clone(),
            BridgeName::PangolinPangoro,
        ))
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
