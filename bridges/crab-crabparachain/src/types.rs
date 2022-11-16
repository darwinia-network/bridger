use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_crab::component::CrabClientComponent;
use client_crab_parachain::component::CrabParachainClientComponent;
use client_kusama::component::KusamaClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::OriginType;
use subquery::SubqueryConfig;

use bin_s2s::bridge::config::RelayConfig;
use bin_s2s::error::{BinS2SError, BinS2SResult};
use bin_s2s::traits::{
    S2SBasicChainInfo, S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo,
    S2SSoloBridgeSoloChainInfo,
};
use support_types::mark::ChainName;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum BridgeFlow {
    KusamaToCrab,
    CrabToCrabParachain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawBridgeConfig {
    pub crab: CrabChainConfig,
    pub kusama: KusamaChainConfig,
    pub crab_parachain: CrabParaChainConfig,
    pub relay: RelayConfig,
    pub index: RawIndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrabChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrabParaChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KusamaChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub crab: SubqueryConfig,
    pub crab_parachain: SubqueryConfig,
    pub kusama: SubqueryConfig,
}

// == solo chain client
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
        Ok(account.account_id().clone())
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

// == para chain client

impl S2SBasicChainInfo for CrabParaChainConfig {
    const CHAIN: ChainName = ChainName::CrabParachain;
}

#[async_trait::async_trait]
impl S2SSoloBridgeSoloChainInfo for CrabParaChainConfig {
    type Client = client_crab_parachain::client::CrabParachainClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgeCrabParachain
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_crab_parachain::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(account.account_id().clone())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_crab_parachain::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(CrabParachainClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

// == relay chain client

impl S2SBasicChainInfo for KusamaChainConfig {
    const CHAIN: ChainName = ChainName::Kusama;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for KusamaChainConfig {
    type Client = client_kusama::client::KusamaClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let relayer_private_key = "//Alice".to_string();
        let config = client_kusama::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key,
            relayer_real_account: None,
        };
        Ok(KusamaClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}
