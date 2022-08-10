use client_crab::client::CrabClient;
use client_crab::component::CrabClientComponent;
use client_darwinia::client::DarwiniaClient;
use client_darwinia::component::DarwiniaClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};

use crate::types::HexLaneId;

/// Bridge template config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    /// Darwinia chain
    pub darwinia: ChainInfoConfig,
    /// Crab chain
    pub crab: ChainInfoConfig,
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
    #[serde(default)]
    pub enable_mandatory: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub crab: SubqueryConfig,
    pub darwinia: SubqueryConfig,
}

impl From<ChainInfoConfig> for client_crab::config::ClientConfig {
    fn from(config: ChainInfoConfig) -> Self {
        client_crab::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.signer,
            relayer_real_account: None,
        }
    }
}

impl From<ChainInfoConfig> for client_darwinia::config::ClientConfig {
    fn from(config: ChainInfoConfig) -> Self {
        client_darwinia::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.signer,
            relayer_real_account: None,
        }
    }
}

impl ChainInfoConfig {
    pub async fn to_crab_client(&self) -> color_eyre::Result<CrabClient> {
        let config = self.clone().into();
        Ok(CrabClientComponent::component(config).await?)
    }

    pub async fn to_darwinia_client(&self) -> color_eyre::Result<DarwiniaClient> {
        let config = self.clone().into();
        Ok(DarwiniaClientComponent::component(config).await?)
    }
}

impl IndexConfig {
    pub fn to_crab_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.crab.clone(), BridgeName::DarwiniaCrab)
    }

    pub fn to_darwinia_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.darwinia.clone(), BridgeName::DarwiniaCrab)
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
