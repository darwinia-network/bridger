use std::cmp;

use crate::strategy::RelayStrategy;
use client_contracts::outbound_types::ReceiveMessagesDeliveryProof;
use client_contracts::{inbound_types::ReceiveMessagesProof, Inbound, Outbound};
use client_contracts::{BeaconLightClient, ExecutionLayer};
use secp256k1::SecretKey;
use subxt::Config;
use support_etherscan::{EtherscanClient, Result as EtherscanResult};
use web3::{transports::Http, types::U256, Web3};

use crate::error::E2EClientResult;

pub trait Web3Client: Send + Sync + Clone {
    // Returns web3 client
    fn get_web3(&self) -> &Web3<Http>;
}

#[async_trait::async_trait]
pub trait GasPriceOracle: Web3Client {
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
    fn beacon_light_client(&self) -> &BeaconLightClient;

    fn execution_layer(&self) -> &ExecutionLayer;

    fn private_key(&self) -> &SecretKey;
}

#[async_trait::async_trait()]
pub trait EcdsaClient: Send + Sync + Clone {
    type SubxtConfig: subxt::Config;

    async fn is_ecdsa_authority(
        &self,
        block_number: Option<u32>,
        your_address: &[u8; 20],
    ) -> E2EClientResult<bool>;

    async fn submit_authorities_change_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> E2EClientResult<<Self::SubxtConfig as Config>::Hash>;

    async fn submit_new_message_root_signature(
        &self,
        address: [u8; 20],
        signatures: Vec<u8>,
    ) -> E2EClientResult<<Self::SubxtConfig as Config>::Hash>;
}

#[async_trait::async_trait]
pub trait MessageClient: GasPriceOracle {
    fn inbound(&self) -> &Inbound;

    fn outbound(&self) -> &Outbound;

    fn private_key(&self) -> &SecretKey;

    async fn decide(&self, encoded_keys: U256) -> E2EClientResult<bool>;

    async fn prepare_for_delivery(&self) -> E2EClientResult<ReceiveMessagesProof>;

    fn delivery_gas_unit(&self) -> E2EClientResult<U256>;

    async fn prepare_for_confirmation(&self) -> E2EClientResult<ReceiveMessagesDeliveryProof>;

    fn confirmation_gas_unit(&self) -> E2EClientResult<U256>;

    async fn latest_light_client_block_number(&self) -> E2EClientResult<Option<u64>>;
}
