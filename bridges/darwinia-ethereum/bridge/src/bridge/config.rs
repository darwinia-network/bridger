use client_contracts::PosaLightClient;
use client_darwinia::client::DarwiniaClient;
use client_darwinia::component::DarwiniaClientComponent;
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
    pub general: GeneralConfig,
    pub darwinia_evm: DarwiniaEVMConfig,
    pub darwinia_substrate: DarwiniaSubstrateConfig,
    pub goerli: ChainInfoConfig,
    pub index: IndexConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub header_goerli_to_darwinia: bool,
    pub sync_commit_goerli_to_darwinia: bool,
    pub execution_layer_goerli_to_darwinia: bool,
    pub ecdsa_service: bool,
    pub msg_goerli_to_darwinia: bool,
    pub msg_darwinia_to_goerli: bool,
    // Max message numbers per delivery
    pub max_message_num_per_relaying: u64,
    // Minium interval(seconds) between every header delivery
    pub header_relay_minimum_interval: u64,
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
    pub max_gas_price: String,
    pub etherscan_api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DarwiniaEVMConfig {
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
    pub max_gas_price: String,
}

impl DarwiniaEVMConfig {
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
pub struct DarwiniaSubstrateConfig {
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

    pub fn to_web3_client(&self) -> color_eyre::Result<Web3<Http>> {
        let transport = Http::new(&self.execution_layer_endpoint)?;
        let client = Web3::new(transport);
        Ok(client)
    }
}

impl From<DarwiniaSubstrateConfig> for client_darwinia::config::ClientConfig {
    fn from(config: DarwiniaSubstrateConfig) -> Self {
        client_darwinia::config::ClientConfig {
            endpoint: config.endpoint,
            relayer_private_key: config.private_key,
            relayer_real_account: None,
        }
    }
}

impl DarwiniaSubstrateConfig {
    pub async fn to_substrate_client(&self) -> color_eyre::Result<DarwiniaClient> {
        let config = self.clone().into();
        Ok(DarwiniaClientComponent::component(config).await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub darwinia: SubqueryConfig,
    pub darwinia_evm: TheGraphLikeEthConfig,
}

impl IndexConfig {
    pub fn to_darwinia_subquery(&self) -> Subquery {
        SubqueryComponent::component(self.darwinia.clone(), BridgeName::DarwiniaEthereum)
    }

    pub fn to_darwinia_thegraph(&self) -> color_eyre::Result<TheGraphLikeEth> {
        Ok(TheGraphLikeEthComponent::component(
            self.darwinia_evm.clone(),
            thegraph_liketh::types::LikethChain::Darwinia,
        )?)
    }
}
