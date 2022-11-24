use std::cmp;

use client_contracts::outbound_types::ReceiveMessagesDeliveryProof;
use client_contracts::{inbound_types::ReceiveMessagesProof, Inbound, Outbound};
use client_contracts::{BeaconLightClient, ExecutionLayer};
use secp256k1::SecretKey;
use subxt::Config;
use support_etherscan::{EtherscanClient, Result as EtherscanResult};
use web3::types::BlockNumber;
use web3::{transports::Http, types::U256, Web3};

use crate::error::E2EClientResult;

pub trait Web3Client: Send + Sync {
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
                let gas_price = U256::from_dec_str(&oracle.propose_gas_price)? * 1_000_000_000i64;
                tracing::trace!(target: "bridge-e2e-traits", "Using etherscan gas price oracle: {:?} Wei", &gas_price);
                gas_price
            }
            None => {
                let gas_price = self.get_web3().eth().gas_price().await?;
                tracing::trace!(target: "bridge-e2e-traits", "Using eth_gasPrice: {:?} Wei", &gas_price);
                gas_price
            }
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
    // Returns the chain name
    fn chain(&self) -> &str;

    // Inbound contract
    fn inbound(&self) -> &Inbound;

    // Outbound contract
    fn outbound(&self) -> &Outbound;

    fn private_key(&self) -> &SecretKey;

    // Retruns true to relay this message, or returns false to not relay this message
    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool>;

    // Returns proof for messages delivery in the range of nonce from begin to end
    async fn prepare_for_delivery(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesProof>;

    // Returns estimated gas used for one message delivery
    fn delivery_gas_unit(&self) -> E2EClientResult<U256>;

    // Returns proof for messages confirmation in the range of nonce from begin to end
    async fn prepare_for_confirmation(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesDeliveryProof>;

    // Returns estimated gas used for one message confirmation
    fn confirmation_gas_unit(&self) -> E2EClientResult<U256>;

    // Returns latest block number of the light client of the other chain
    async fn latest_light_client_block_number(&self) -> E2EClientResult<Option<u64>>;
}
