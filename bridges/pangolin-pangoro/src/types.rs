use bin_s2s::bridge::config::RelayConfig;
use bin_s2s::error::{BinS2SError, BinS2SResult};
use bin_s2s::traits::{S2SBasicChainInfo, S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo};
use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_pangoro::component::PangoroClientComponent;
use client_pangolin::component::PangolinClientComponent;
use client_moonbase::component::MoonbaseClientComponent;
use client_rococo::component::RococoClientComponent;
use serde::{Deserialize, Serialize};
use subquery::types::OriginType;
use subquery::SubqueryConfig;

use support_types::mark::ChainName;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, strum::EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum BridgeFlow {
    PangolinToPangoro,
    PangoroToPangolin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawBridgeConfig {
    pub pangolin: PangolinChainConfig,
    pub pangoro: PangoroChainConfig,
    pub rococo: RococoChainConfig,
    pub moonbase: MoonbaseChainConfig,
    pub relay: RelayConfig,
    pub index: RawIndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
    pub para_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangoroChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
    pub para_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoonbaseChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RococoChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub pangolin: SubqueryConfig,
    pub pangoro: SubqueryConfig,
    pub rococo: SubqueryConfig,
    pub moonbase: SubqueryConfig,
}

// === parachain

impl S2SBasicChainInfo for PangolinChainConfig {
    const CHAIN: ChainName = ChainName::Pangolin;
}

#[async_trait::async_trait]
impl S2SParaBridgeSoloChainInfo for PangolinChainConfig {
    type Client = client_pangolin::client::PangolinClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgePangolin
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_pangolin::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?;
        Ok(*account.account_id())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_pangolin::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangolinClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?)
    }
}

impl S2SBasicChainInfo for PangoroChainConfig {
    const CHAIN: ChainName = ChainName::Pangoro;
}

#[async_trait::async_trait]
impl S2SParaBridgeSoloChainInfo for PangoroChainConfig {
    type Client = client_pangoro::client::PangoroClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgePangoro
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account = client_pangoro::types::DarwiniaAccount::new(self.signer.clone(), None)
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?;
        Ok(*account.account_id())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_pangoro::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangoroClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?)
    }
}

// === relay chain

impl S2SBasicChainInfo for RococoChainConfig {
    const CHAIN: ChainName = ChainName::Rococo;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for RococoChainConfig {
    type Client = client_rococo::client::RococoClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_rococo::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: "//Alice".to_string(),
        };
        Ok(RococoClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?)
    }
}

impl S2SBasicChainInfo for MoonbaseChainConfig {
    const CHAIN: ChainName = ChainName::Moonbase;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for MoonbaseChainConfig {
    type Client = client_moonbase::client::MoonbaseClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_moonbase::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: "//Alice".to_string(),
        };
        Ok(MoonbaseClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{e:?}")))?)
    }
}
