use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_pangolin::component::PangolinClientComponent;
use client_pangolin_parachain::component::PangolinParachainClientComponent;
use client_rococo::component::RococoClientComponent;
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
    RococoToPangolin,
    PangolinToPangolinParachain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawBridgeConfig {
    pub pangolin: PangolinChainConfig,
    pub rococo: RococoChainConfig,
    pub pangolin_parachain: PangolinParaChainConfig,
    pub relay: RelayConfig,
    pub index: RawIndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinParaChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RococoChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub pangolin: SubqueryConfig,
    pub pangolin_parachain: SubqueryConfig,
    pub rococo: SubqueryConfig,
}

// == solo chain client
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
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(account.account_id().clone())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_pangolin::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangolinClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

// == para chain client

impl S2SBasicChainInfo for PangolinParaChainConfig {
    const CHAIN: ChainName = ChainName::PangolinParachain;
}

#[async_trait::async_trait]
impl S2SSoloBridgeSoloChainInfo for PangolinParaChainConfig {
    type Client = client_pangolin_parachain::client::PangolinParachainClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgePangolinParachain
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account =
            client_pangolin_parachain::types::DarwiniaAccount::new(self.signer.clone(), None)
                .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(account.account_id().clone())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_pangolin_parachain::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(PangolinParachainClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

// == relay chain client

impl S2SBasicChainInfo for RococoChainConfig {
    const CHAIN: ChainName = ChainName::Rococo;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for RococoChainConfig {
    type Client = client_rococo::client::RococoClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let relayer_private_key = "//Alice".to_string();
        let config = client_rococo::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key,
            relayer_real_account: None,
        };
        Ok(RococoClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}
