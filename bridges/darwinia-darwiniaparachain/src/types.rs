use bridge_s2s_traits::types::bp_runtime;
use client_common_traits::ClientCommon;
use client_darwinia::component::DarwiniaClientComponent;
use client_darwinia_parachain::component::DarwiniaParachainClientComponent;
use client_polkadot::component::PolkadotClientComponent;
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
    PolkadotToDarwinia,
    DarwiniaToDarwiniaParachain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawBridgeConfig {
    pub darwinia: DarwiniaChainConfig,
    pub polkadot: PolkadotChainConfig,
    pub darwinia_parachain: DarwiniaParaChainConfig,
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
pub struct DarwiniaParaChainConfig {
    /// Endpoint
    pub endpoint: String,
    pub signer: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolkadotChainConfig {
    /// Endpoint
    pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawIndexConfig {
    pub darwinia: SubqueryConfig,
    pub darwinia_parachain: SubqueryConfig,
    pub polkadot: SubqueryConfig,
}

// == solo chain client
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

impl S2SBasicChainInfo for DarwiniaParaChainConfig {
    const CHAIN: ChainName = ChainName::DarwiniaParachain;
}

#[async_trait::async_trait]
impl S2SSoloBridgeSoloChainInfo for DarwiniaParaChainConfig {
    type Client = client_darwinia_parachain::client::DarwiniaParachainClient;

    fn origin_type(&self) -> OriginType {
        OriginType::BridgeDarwiniaParachain
    }

    fn account(
        &self,
    ) -> BinS2SResult<<<Self::Client as ClientCommon>::Chain as bp_runtime::Chain>::AccountId> {
        let account =
            client_darwinia_parachain::types::DarwiniaAccount::new(self.signer.clone(), None)
                .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?;
        Ok(account.account_id().clone())
    }

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let config = client_darwinia_parachain::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key: self.signer.clone(),
            relayer_real_account: None,
        };
        Ok(DarwiniaParachainClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}

// == relay chain client

impl S2SBasicChainInfo for PolkadotChainConfig {
    const CHAIN: ChainName = ChainName::Polkadot;
}

#[async_trait::async_trait]
impl S2SParaBridgeRelayChainInfo for PolkadotChainConfig {
    type Client = client_polkadot::client::PolkadotClient;

    async fn client(&self) -> BinS2SResult<Self::Client> {
        let relayer_private_key = "//Alice".to_string();
        let config = client_polkadot::config::ClientConfig {
            endpoint: self.endpoint.clone(),
            relayer_private_key,
            relayer_real_account: None,
        };
        Ok(PolkadotClientComponent::component(config)
            .await
            .map_err(|e| BinS2SError::Client(format!("{:?}", e)))?)
    }
}
