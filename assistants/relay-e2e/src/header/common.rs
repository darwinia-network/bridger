use bridge_e2e_traits::client::{EthTruthLayerLightClient, GasPriceOracle, Web3Client};
use client_contracts::{beacon_light_client::BeaconLightClient, ExecutionLayer};
use secp256k1::SecretKey;
use std::str::FromStr;
use web3::{
    transports::Http,
    types::{Address, U256},
    Web3,
};

use crate::error::{RelayError, RelayResult};

#[derive(Debug, Clone)]
pub struct EthLightClient {
    pub client: Web3<Http>,
    pub beacon_light_client: BeaconLightClient,
    pub execution_layer: ExecutionLayer,
    pub private_key: SecretKey,
    pub max_gas_price: U256,
}

impl EthLightClient {
    pub fn new(
        endpoint: &str,
        contract_address: &str,
        execution_layer_contract_address: &str,
        private_key: &str,
        max_gas_price: U256,
    ) -> RelayResult<Self> {
        let transport = Http::new(endpoint)?;
        let client = web3::Web3::new(transport);
        let beacon_light_client = BeaconLightClient::new(
            &client,
            Address::from_str(contract_address)
                .map_err(|e| RelayError::Custom(format!("{}", e)))?,
        )?;
        let execution_layer = ExecutionLayer::new(
            &client,
            Address::from_str(execution_layer_contract_address)
                .map_err(|e| RelayError::Custom(format!("{}", e)))?,
        )?;
        let private_key = SecretKey::from_str(private_key)?;
        Ok(Self {
            client,
            beacon_light_client,
            execution_layer,
            private_key,
            max_gas_price,
        })
    }
}

impl Web3Client for EthLightClient {
    fn get_web3(&self) -> &Web3<Http> {
        &self.client
    }
}

impl EthTruthLayerLightClient for EthLightClient {
    fn beacon_light_client(&self) -> &BeaconLightClient {
        &self.beacon_light_client
    }

    fn execution_layer(&self) -> &ExecutionLayer {
        &self.execution_layer
    }

    fn private_key(&self) -> &SecretKey {
        &self.private_key
    }
}

impl GasPriceOracle for EthLightClient {
    fn get_etherscan_client(&self) -> Option<&support_etherscan::EtherscanClient> {
        None
    }

    fn max_gas_price(&self) -> U256 {
        self.max_gas_price
    }
}
