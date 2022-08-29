use client_contracts::PosaLightClient;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use relay_e2e::types::ethereum::FastEthereumAccount;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};
use thegraph_liketh::component::TheGraphLikeEthComponent;
use thegraph_liketh::config::TheGraphLikeEthConfig;
use thegraph_liketh::graph::TheGraphLikeEth;
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig {
    pub pangoro_evm: PangoroEVMConfig,
    pub pangoro_substrate: PangoroSubstrateConfig,
    pub goerli: ChainInfoConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainInfoConfig {
    pub endpoint: String,
    pub execution_layer_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    pub account: String,
    pub private_key: String,
    pub inbound_address: String,
    pub outbound_address: String,
    pub fee_market_address: String,
    pub posa_light_client_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PangoroEVMConfig {
    pub endpoint: String,
    pub contract_address: String,
    pub execution_layer_contract_address: String,
    pub account: String,
    pub private_key: String,
    pub inbound_address: String,
    pub outbound_address: String,
    pub chain_message_committer_address: String,
    pub lane_message_committer_address: String,
    pub fee_market_address: String,
}

impl PangoroEVMConfig {
    pub fn to_fast_ethereum_account(&self) -> FastEthereumAccount {
        FastEthereumAccount::new(&self.private_key)
    }

    pub fn to_web3_client(&self) -> color_eyre::Result<Web3<Http>> {
        let transport = Http::new(&self.endpoint)?;
        let client = Web3::new(transport);
        Ok(client)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PangoroSubstrateConfig {
    pub endpoint: String,
    pub private_key: String,
}

impl ChainInfoConfig {
    pub fn to_posa_client(&self) -> color_eyre::Result<PosaLightClient> {
        let transport = Http::new(&self.execution_layer_endpoint)?;
        let client = Web3::new(transport);
        let address = Address::from_str(&self.posa_light_client_address)?;
        Ok(PosaLightClient::new(client, address)?)
    }
    pub fn to_ethereum_account(&self) -> FastEthereumAccount {
        FastEthereumAccount::new(&self.private_key)
    }
}

impl From<PangoroSubstrateConfig> for client_pangoro::config::ClientConfig {
    fn from(config: PangoroSubstrateConfig) -> Self {
        client_pangoro::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.private_key,
            relayer_real_account: None,
        }
    }
}

impl PangoroSubstrateConfig {
    pub async fn to_substrate_client(&self) -> color_eyre::Result<PangoroClient> {
        let config = self.clone().into();
        Ok(PangoroClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub pangoro: SubqueryConfig,
    pub pangoro_evm: TheGraphLikeEthConfig,
}

impl IndexConfig {
    pub fn to_pangoro_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.pangoro.clone(), BridgeName::PangoroGoerli)
    }

    pub fn to_pangoro_thegraph(&self) -> color_eyre::Result<TheGraphLikeEth> {
        Ok(TheGraphLikeEthComponent::component(
            self.pangoro_evm.clone(),
            thegraph_liketh::types::LikethChain::Pangoro,
        )?)
    }
}
