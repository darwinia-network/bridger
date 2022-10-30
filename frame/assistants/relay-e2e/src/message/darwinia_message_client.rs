use std::str::FromStr;

use bridge_e2e_traits::{
    client::{EthTruthLayerLightClient, GasPriceOracle, MessageClient, Web3Client},
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use client_beacon::client::BeaconApiClient;
use client_contracts::{
    inbound_types::ReceiveMessagesProof, outbound_types::ReceiveMessagesDeliveryProof,
    ChainMessageCommitter, Inbound, LaneMessageCommitter, Outbound,
};
use secp256k1::SecretKey;
use thegraph_liketh::graph::TheGraphLikeEth;
use web3::{
    transports::Http,
    types::{H256, U256},
    Web3,
};

use crate::header::common::EthLightClient;

pub struct DarwiniaMessageClient<T: RelayStrategy> {
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
    pub chain_message_committer: ChainMessageCommitter,
    pub lane_message_committer: LaneMessageCommitter,
    pub strategy: T,
    pub private_key: SecretKey,
    pub indexer: TheGraphLikeEth,

    pub beacon_rpc_client: BeaconApiClient,
    pub eth_light_client: EthLightClient,
}

impl<T: RelayStrategy> Web3Client for DarwiniaMessageClient<T> {
    fn get_web3(&self) -> &Web3<Http> {
        &self.client
    }
}

impl<T: RelayStrategy> GasPriceOracle for DarwiniaMessageClient<T> {
    fn get_etherscan_client(&self) -> Option<&support_etherscan::EtherscanClient> {
        None
    }

    fn max_gas_price(&self) -> web3::types::U256 {
        self.eth_light_client.max_gas_price
    }
}

#[async_trait::async_trait]
impl<T: RelayStrategy> MessageClient for DarwiniaMessageClient<T> {
    fn inbound(&self) -> &Inbound {
        &self.inbound
    }

    fn outbound(&self) -> &Outbound {
        &self.outbound
    }

    fn private_key(&self) -> &SecretKey {
        &self.private_key
    }

    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool> {
        self.strategy.decide(encoded_key).await
    }

    async fn prepare_for_delivery(&self) -> E2EClientResult<ReceiveMessagesProof> {
        todo!()
    }

    fn delivery_gas_unit(&self) -> E2EClientResult<U256> {
        Ok(U256::from_dec_str("1000000").map_err(|e| E2EClientError::Custom(format!("{}", e)))?)
    }

    async fn prepare_for_confirmation(&self) -> E2EClientResult<ReceiveMessagesDeliveryProof> {
        todo!()
    }

    fn confirmation_gas_unit(&self) -> E2EClientResult<U256> {
        Ok(U256::from_dec_str("10000000").map_err(|e| E2EClientError::Custom(format!("{}", e)))?)
    }

    async fn latest_light_client_block_number(&self) -> E2EClientResult<Option<u64>> {
        let header = self
            .eth_light_client
            .beacon_light_client()
            .finalized_header()
            .await?;
        let block = self
            .beacon_rpc_client
            .get_beacon_block(header.slot)
            .await
            .map_err(|e| E2EClientError::Custom(format!("Beacon api error: {}", e)))?;
        let execution_state_root = self
            .eth_light_client
            .execution_layer()
            .merkle_root(None)
            .await?;
        let latest_state_root = H256::from_str(&block.body.execution_payload.state_root)
            .map_err(|e| E2EClientError::Custom(format!("{}", e)))?;
        if execution_state_root != latest_state_root {
            Ok(None)
        } else {
            Ok(Some(
                block
                    .body
                    .execution_payload
                    .block_number
                    .parse()
                    .map_err(|e| E2EClientError::Custom(format!("{}", e)))?,
            ))
        }
    }
}
