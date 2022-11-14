use client_darwinia::client::DarwiniaClient;
use client_darwinia::component::DarwiniaClientComponent;
use client_darwinia_parachain::client::DarwiniaParachainClient;
use client_darwinia_parachain::component::DarwiniaParachainClientComponent;
use client_polkadot::client::PolkadotClient;
use client_polkadot::component::PolkadotClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};

use support_common::error::BridgerError;

use crate::types::HexLaneId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub darwinia: ChainInfoConfig,
    pub polkadot: ChainInfoConfig,
    pub darwinia_parachain: ChainInfoConfig,
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

impl TryFrom<ChainInfoConfig> for client_darwinia::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_darwinia::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_darwinia_parachain::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = config.signer.ok_or_else(|| {
            BridgerError::Custom(format!("Missing signer for chain: {}", config.endpoint))
        })?;
        Ok(client_darwinia_parachain::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl TryFrom<ChainInfoConfig> for client_polkadot::config::ClientConfig {
    type Error = BridgerError;

    fn try_from(config: ChainInfoConfig) -> Result<Self, Self::Error> {
        let relayer_private_key = "//Alice".to_string();
        Ok(client_polkadot::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key,
            relayer_real_account: None,
        })
    }
}

impl ChainInfoConfig {
    pub async fn to_darwinia_client(&self) -> color_eyre::Result<DarwiniaClient> {
        let config = self.clone().try_into()?;
        Ok(DarwiniaClientComponent::component(config).await?)
    }

    pub async fn to_darwinia_parachain_client(
        &self,
    ) -> color_eyre::Result<DarwiniaParachainClient> {
        let config = self.clone().try_into()?;
        Ok(DarwiniaParachainClientComponent::component(config).await?)
    }

    pub async fn to_polkadot_client(&self) -> color_eyre::Result<PolkadotClient> {
        let config = self.clone().try_into()?;
        Ok(PolkadotClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub darwinia: SubqueryConfig,
    pub darwinia_parachain: SubqueryConfig,
    pub polkadot: SubqueryConfig,
}

impl IndexConfig {
    pub fn to_darwinia_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.darwinia.clone(), BridgeName::DarwiniaParachain)
    }

    pub fn to_darwinia_parachain_subquery(&self) -> Subquery {
        SubqueryComponent::component(
            self.darwinia_parachain.clone(),
            BridgeName::DarwiniaParachain,
        )
    }

    pub fn to_polkadot_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.polkadot.clone(), BridgeName::DarwiniaParachain)
    }
}

impl RelayConfig {
    pub fn raw_lanes(&self) -> Vec<[u8; 4]> {
        self.lanes.iter().map(|item| item.0).collect()
    }
}
