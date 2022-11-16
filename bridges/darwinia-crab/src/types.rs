use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_crab::component::CrabClientComponent;
use client_darwinia::component::DarwiniaClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::OriginType;
use subquery::SubqueryConfig;

use bin_s2s::bridge::config::RelayConfig;
use bin_s2s::error::{BinS2SError, BinS2SResult};
use bin_s2s::traits::{S2SBasicChainInfo, S2SSoloBridgeSoloChainInfo};
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
    pub relay: RelayConfig,
    pub index: RawIndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DarwiniaChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrabChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub darwinia: SubqueryConfig,
    pub crab: SubqueryConfig,
}

// == solo chain client
impl S2SBasicChainInfo for DarwiniaChainConfig {
    const CHAIN: ChainName = ChainName::Darwinia;
}

#[async_trait::async_trait]
impl S2SSoloBridgeSoloChainInfo for DarwiniaChainConfig {
    type Client = client_darwinia::client::DarwiniaClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgeDarwinia
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_darwinia::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(account.account_id().clone())
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

// == para chain client

impl S2SBasicChainInfo for CrabChainConfig {
    const CHAIN: ChainName = ChainName::Crab;
}

#[async_trait::async_trait]
impl S2SSoloBridgeSoloChainInfo for CrabChainConfig {
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
