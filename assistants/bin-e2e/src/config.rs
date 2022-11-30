use std::fmt::Display;
use std::str::FromStr;

use bridge_e2e_traits::client::EcdsaClient;
use client_beacon::client::ApiSupplier;
use client_contracts::PosaLightClient;
use relay_e2e::types::ethereum::FastEthereumAccount;
use serde::{Deserialize, Deserializer, Serialize};
use subquery::types::BridgeName;
use subquery::{Subquery, SubqueryComponent, SubqueryConfig};
use thegraph::Thegraph;
use thegraph::ThegraphComponent;
use thegraph::ThegraphConfig;
use web3::transports::Http;
use web3::types::Address;
use web3::Web3;

use crate::bridge::BridgeBus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BridgeConfig<T: EcdsaClient> {
    pub general: GeneralConfig,
    pub darwinia_evm: EVMChainConfig,
    pub substrate_client: T,
    pub ethereum: ExecutionLayerInfoConfig,
    pub beacon: BeaconApiConfig,
    pub index: IndexConfig,
}

impl<T: EcdsaClient> lifeline::Storage for BridgeConfig<T> {
    fn take_or_clone(res: &mut Option<Self>) -> Option<Self> {
        res.clone()
    }
}
impl<T: EcdsaClient> lifeline::Resource<BridgeBus> for BridgeConfig<T> {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub enable_beacon_header_relay: bool,
    pub enable_sync_commit_relay: bool,
    pub enable_execution_header_layer: bool,
    pub enable_ecdsa_relay: bool,
    pub enable_message_execution_to_evm: bool,
    pub enable_message_evm_to_execution: bool,
    // Max message numbers per delivery
    pub max_message_num_per_relaying: u64,
    // Minium interval(seconds) between every header delivery
    pub header_relay_minimum_interval: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionLayerInfoConfig {
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
    #[serde(deserialize_with = "evm_secret_key_from_str")]
    pub private_key: String,
    pub inbound_address: String,
    pub outbound_address: String,
    pub fee_market_address: String,
    pub posa_light_client_address: String,
    pub max_gas_price: String,
    pub etherscan_api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeaconApiConfig {
    pub endpoint: String,
    pub api_supplier: ApiSupplier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EVMChainConfig {
    pub endpoint: String,
    pub contract_address: String,
    pub execution_layer_contract_address: String,
    #[serde(deserialize_with = "evm_secret_key_from_str")]
    pub private_key: String,
    pub inbound_address: String,
    pub outbound_address: String,
    pub chain_message_committer_address: String,
    pub lane_message_committer_address: String,
    pub fee_market_address: String,
    pub max_gas_price: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub substrate_chain: SubqueryConfig,
    pub evm_chain: ThegraphConfig,
}

impl EVMChainConfig {
    pub fn to_ethereum_account(&self) -> FastEthereumAccount {
        FastEthereumAccount::new(&self.private_key)
    }

    pub fn to_web3_client(&self) -> color_eyre::Result<Web3<Http>> {
        let transport = Http::new(&self.endpoint)?;
        let client = Web3::new(transport);
        Ok(client)
    }
}

impl ExecutionLayerInfoConfig {
    pub fn to_posa_client(&self) -> color_eyre::Result<PosaLightClient> {
        let transport = Http::new(&self.endpoint)?;
        let client = Web3::new(transport);
        let address = Address::from_str(&self.posa_light_client_address)?;
        Ok(PosaLightClient::new(&client, address)?)
    }

    pub fn to_ethereum_account(&self) -> FastEthereumAccount {
        FastEthereumAccount::new(&self.private_key)
    }

    pub fn to_web3_client(&self) -> color_eyre::Result<Web3<Http>> {
        let transport = Http::new(&self.endpoint)?;
        let client = Web3::new(transport);
        Ok(client)
    }
}

impl IndexConfig {
    pub fn to_substrate_subquery(&self, bridge_name: BridgeName) -> Subquery {
        SubqueryComponent::component(self.substrate_chain.clone(), bridge_name)
    }

    pub fn to_evm_thegraph(
        &self,
        chain: thegraph::types::LikethChain,
    ) -> color_eyre::Result<Thegraph> {
        Ok(ThegraphComponent::component(self.evm_chain.clone(), chain)?)
    }
}

fn evm_secret_key_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.replace("0x", "");
    T::from_str(&s).map_err(serde::de::Error::custom)
}
