use client_crab::client::CrabClient;
use client_crab::component::CrabClientComponent;
use client_crab_parachain::client::CrabParachainClient;
use client_crab_parachain::component::CrabParachainClientComponent;
use client_kusama::client::KusamaClient;
use client_kusama::component::KusamaClientComponent;
use serde::{Deserialize, Serialize};
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent, SubqueryConfig};

use support_common::error::BridgerError;

use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub crab: ChainInfoConfig,
    pub kusama: ChainInfoConfig,
    pub crab_parachain: ChainInfoConfig,
    pub relay: RelayConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    pub para_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    /// Endpoint
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
}

impl TryFrom<ChainInfoConfig> for client_crab::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_crab::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_crab_parachain::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_crab_parachain::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_kusama::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = "//Alice".to_string();
        Ok(client_kusama::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl ChainInfoConfig {
    pub async fn to_crab_client(&self) -> color_eyre::Result<CrabClient> {
        let config = self.clone().try_into()?;
        Ok(CrabClientComponent::component(config).await?)
    }

    pub async fn to_crab_parachain_client(&self) -> color_eyre::Result<CrabParachainClient> {
        let config = self.clone().try_into()?;
        Ok(CrabParachainClientComponent::component(config).await?)
    }

    pub async fn to_kusama_client(&self) -> color_eyre::Result<KusamaClient> {
        let config = self.clone().try_into()?;
        Ok(KusamaClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub crab: SubqueryConfig,
    pub crab_parachain: SubqueryConfig,
    pub kusama: SubqueryConfig,
    pub parachain_kusama: subquery_parachain::SubqueryConfig,
}

impl IndexConfig {
    pub fn to_crab_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.crab.clone(), BridgeName::CrabParachain)
    }

    pub fn to_crab_parachain_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.crab_parachain.clone(), BridgeName::CrabParachain)
    }

    pub fn to_kusama_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.kusama.clone(), BridgeName::CrabParachain)
    }

    pub fn to_candidate_subquery(&self) -> subquery_parachain::Subquery {
        subquery_parachain::SubqueryComponent::component(
            self.parachain_kusama.clone(),
            subquery_parachain::types::BridgeName::CrabParachain,
        )
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
