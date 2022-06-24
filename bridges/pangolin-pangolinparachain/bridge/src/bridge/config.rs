use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin_parachain::client::PangolinParachainClient;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use client_rococo::client::RococoClient;
use client_rococo::component::RococoClientComponent;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent, SubqueryConfig};

use component_subscan::SubscanConfig;
use support_common::error::BridgerError;

use crate::types::{ChainInfo, HexLaneId, PrometheusParamsInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub pangolin: ChainInfoConfig,
    pub rococo: ChainInfoConfig,
    pub pangolin_parachain: ChainInfoConfig,
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

impl TryFrom<ChainInfoConfig> for client_pangolin::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_pangolin::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_pangolin_parachain::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_pangolin_parachain::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_rococo::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = "//Alice".to_string();
        Ok(client_rococo::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl ChainInfoConfig {
    pub async fn to_pangolin_client(&self) -> color_eyre::Result<PangolinClient> {
        let config = self.try_into()?;
        Ok(PangolinClientComponent::component(config).await?)
    }

    pub async fn to_pangolin_parachain_client(
        &self,
    ) -> color_eyre::Result<PangolinParachainClient> {
        let config = self.try_into()?;
        Ok(PangolinParachainClientComponent::component(config).await?)
    }

    pub async fn to_rococo_client(&self) -> color_eyre::Result<RococoClient> {
        let config = self.try_into()?;
        Ok(RococoClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub pangolin: SubqueryConfig,
    pub pangolin_parachain: SubqueryConfig,
    pub rococo: SubqueryConfig,
    pub parachain_rococo: subquery_parachain::SubqueryConfig,
}

impl IndexConfig {
    pub fn to_pangolin_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.pangolin.clone(), BridgeName::PangolinParachain)
    }

    pub fn to_pangolin_parachain_subquery(&self) -> Subquery {
        SubqueryComponent::component(
            self.pangolin_parachain.clone(),
            BridgeName::PangolinParachain,
        )
    }

    pub fn to_rococo_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.rococo.clone(), BridgeName::PangolinParachain)
    }

    pub fn to_candidate_subquery(&self) -> subquery_parachain::Subquery {
        subquery_parachain::SubqueryComponent::component(
            self.parachain_rococo.clone(),
            subquery_parachain::types::BridgeName::PangolinParachain,
        )
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
