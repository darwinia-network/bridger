use std::cmp;

use client_contracts::{BeaconLightClient, ExecutionLayer};
use secp256k1::SecretKey;
use support_etherscan::{EtherscanClient, Result as EtherscanResult};
use web3::{transports::Http, types::U256, Web3};

#[async_trait::async_trait]
pub trait GasPriceOracle {
    // Returns web3 client
    fn get_web3(&self) -> &Web3<Http>;

    // Returns etherscan api client
    fn get_etherscan_client(&self) -> Option<&EtherscanClient>;

    // Returns Max gas price that GasPriceOracle should return
    fn max_gas_price(&self) -> U256;

    // Returns gas price
    async fn gas_price(&self) -> EtherscanResult<U256> {
        let price: U256 = match self.get_etherscan_client() {
            Some(etherscan_client) => {
                let oracle = etherscan_client.get_gas_oracle().await?;
                U256::from_dec_str(&oracle.propose_gas_price)? * 1_000_000_000i64
            }
            None => self.get_web3().eth().gas_price().await?,
        };
        Ok(cmp::min(self.max_gas_price(), price))
    }
}

pub trait EthTruthLayerLightClient: GasPriceOracle {
    fn web3_client(&self) -> &Web3<Http>;

    fn beacon_light_client(&self) -> &BeaconLightClient;

    fn execution_layer(&self) -> &ExecutionLayer;

    fn private_key(&self) -> &SecretKey;
}
