use bin_s2s::bridge::config::RelayConfig;
use bin_s2s::error::{BinS2SError, BinS2SResult};
use bin_s2s::traits::{S2SBasicChainInfo, S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo};
use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_crab::component::CrabClientComponent;
use client_darwinia::component::DarwiniaClientComponent;
use client_kusama::component::KusamaClientComponent;
use client_polkadot::component::PolkadotClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::OriginType;
use subquery::SubqueryConfig;

use support_types::mark::ChainName;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum BridgeFlow {
    DarwiniaToCrab,
    CrabToDarwinia,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawBridgeConfig {
    pub darwinia: DarwiniaChainConfig,
    pub crab: CrabChainConfig,
    pub polkadot: PolkadotChainConfig,
    pub kusama: KusamaChainConfig,
    pub relay: RelayConfig,
    pub index: RawIndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
    pub para_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrabChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
    pub para_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KusamaChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolkadotChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub darwinia: SubqueryConfig,
    pub crab: SubqueryConfig,
    pub polkadot: SubqueryConfig,
    pub kusama: SubqueryConfig,
}

// === parachain

impl S2SBasicChainInfo for DarwiniaChainConfig {
    const CHAIN: ChainName = ChainName::Darwinia;
}

#[async_trait::async_trait]
impl S2SParaBridgeSoloChainInfo for DarwiniaChainConfig {
    type Client = client_darwinia::client::DarwiniaClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgeDarwinia
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_darwinia::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(*account.account_id())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_darwinia::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(DarwiniaClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

impl S2SBasicChainInfo for CrabChainConfig {
    const CHAIN: ChainName = ChainName::Crab;
}

#[async_trait::async_trait]
impl S2SParaBridgeSoloChainInfo for CrabChainConfig {
    type Client = client_crab::client::CrabClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgeCrab
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_crab::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(*account.account_id())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_crab::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(CrabClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

// === relay chain

impl S2SBasicChainInfo for PolkadotChainConfig {
    const CHAIN: ChainName = ChainName::Polkadot;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for PolkadotChainConfig {
    type Client = client_polkadot::client::PolkadotClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_polkadot::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: "//Alice".to_string(),
            relayer_real_account: None,
        };
        Ok(PolkadotClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

impl S2SBasicChainInfo for KusamaChainConfig {
    const CHAIN: ChainName = ChainName::Kusama;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for KusamaChainConfig {
    type Client = client_kusama::client::KusamaClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_kusama::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: "//Alice".to_string(),
            relayer_real_account: None,
        };
        Ok(KusamaClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}
