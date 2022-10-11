use client_moonbase::client::MoonbaseClient;
use client_moonbase::component::MoonbaseClientComponent;
use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin_parachain::client::PangolinParachainClient;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};

use support_common::error::BridgerError;

use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub pangolin: ChainInfoConfig,
    pub moonbase: ChainInfoConfig,
    pub pangolin_parachain_alpha: ChainInfoConfig,
    pub relay: RelayConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Hex-encoded lane identifiers that should be served by the complex relay.
    pub lanes: Vec<HexLaneId>,
    pub para_id: u32,
    #[serde(default)]
    pub enable_mandatory: bool,
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

impl TryFrom<ChainInfoConfig> for client_moonbase::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = "//Alice".to_string();
        Ok(client_moonbase::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl ChainInfoConfig {
    pub async fn to_pangolin_client(&self) -> color_eyre::Result<PangolinClient> {
        let config = self.clone().try_into()?;
        Ok(PangolinClientComponent::component(config).await?)
    }

    pub async fn to_pangolin_parachain_client(
        &self,
    ) -> color_eyre::Result<PangolinParachainClient> {
        let config = self.clone().try_into()?;
        Ok(PangolinParachainClientComponent::component(config).await?)
    }

    pub async fn to_moonbase_client(&self) -> color_eyre::Result<MoonbaseClient> {
        let config = self.clone().try_into()?;
        Ok(MoonbaseClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub pangolin: SubqueryConfig,
    pub pangolin_parachain_alpha: SubqueryConfig,
    pub moonbase: SubqueryConfig,
}

impl IndexConfig {
    pub fn to_pangolin_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.pangolin.clone(), BridgeName::PangolinParachainAlpha)
    }

    pub fn to_pangolin_parachain_subquery(&self) -> Subquery {
        SubqueryComponent::component(
            self.pangolin_parachain_alpha.clone(),
            BridgeName::PangolinParachainAlpha,
        )
    }

    pub fn to_moonbase_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.moonbase.clone(), BridgeName::PangolinParachainAlpha)
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
