use std::str::FromStr;

use bridge_e2e_traits::{
    client::{EthTruthLayerLightClient, GasPriceOracle, MessageClient, Web3Client},
    error::{E2EClientError, E2EClientResult},
    strategy::RelayStrategy,
};
use client_beacon::client::{ApiSupplier, BeaconApiClient};
use client_contracts::{
    inbound_types::{Message, OutboundLaneData, Payload, ReceiveMessagesProof},
    outbound_types::{MessageAccepted, ReceiveMessagesDeliveryProof},
    ChainMessageCommitter, FeeMarket, Inbound, LaneMessageCommitter, Outbound,
};
use secp256k1::SecretKey;
use thegraph::Thegraph;
use web3::{
    contract::tokens::Tokenizable,
    ethabi::encode,
    signing::Key,
    transports::Http,
    types::{Address, BlockId, BlockNumber, Bytes, H256, U256},
    Web3,
};

use super::fee_market::FeeMarketRelayStrategy;
use crate::header::common::EthLightClient;

pub struct DarwiniaMessageClient<T: RelayStrategy = FeeMarketRelayStrategy> {
    pub chain: String,
    pub client: Web3<Http>,
    pub inbound: Inbound,
    pub outbound: Outbound,
    pub chain_message_committer: ChainMessageCommitter,
    pub lane_message_committer: LaneMessageCommitter,
    pub strategy: T,
    pub indexer: Thegraph,
    pub beacon_rpc_client: BeaconApiClient,
    pub eth_light_client: EthLightClient,
}

impl DarwiniaMessageClient {
    pub fn new_with_fee_market(
        chain: &str,
        endpoint: &str,
        beacon_api_endpoint: &str,
        beacon_api_supplier: ApiSupplier,
        inbound_address: Address,
        outbound_address: Address,
        chain_message_committer_address: Address,
        lane_message_committer_address: Address,
        fee_market_address: Address,
        light_client_address: Address,
        execution_layer_address: Address,
        max_gas_price: U256,
        private_key: &str,
        indexer: Thegraph,
    ) -> E2EClientResult<DarwiniaMessageClient> {
        let transport = Http::new(endpoint)?;
        let client = Web3::new(transport);

        let beacon_rpc_client = BeaconApiClient::new(beacon_api_endpoint, beacon_api_supplier)
            .map_err(|_| E2EClientError::Custom("Failed to build beacon api client".into()))?;

        let inbound = Inbound::new(&client, inbound_address)?;
        let outbound = Outbound::new(&client, outbound_address)?;
        let fee_market = FeeMarket::new(&client, fee_market_address)?;
        let chain_message_committer =
            ChainMessageCommitter::new(&client, chain_message_committer_address)?;
        let lane_message_committer =
            LaneMessageCommitter::new(&client, lane_message_committer_address)?;

        let eth_light_client = EthLightClient::new(
            endpoint,
            light_client_address,
            execution_layer_address,
            private_key,
            max_gas_price,
        )
        .map_err(|e| E2EClientError::Custom(format!("Failed to build EthLightClient: {}", e)))?;
        let account = eth_light_client.private_key().address();
        let strategy = FeeMarketRelayStrategy::new(fee_market, account);

        Ok(DarwiniaMessageClient {
            chain: chain.into(),
            client,
            inbound,
            outbound,
            chain_message_committer,
            lane_message_committer,
            strategy,
            indexer,
            beacon_rpc_client,
            eth_light_client,
        })
    }
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
    fn chain(&self) -> &str {
        &self.chain
    }

    fn inbound(&self) -> &Inbound {
        &self.inbound
    }

    fn outbound(&self) -> &Outbound {
        &self.outbound
    }

    fn private_key(&self) -> &SecretKey {
        &self.eth_light_client.private_key
    }

    async fn decide(&mut self, encoded_key: U256) -> E2EClientResult<bool> {
        self.strategy.decide(encoded_key).await
    }

    async fn prepare_for_delivery(
        &self,
        begin: u64,
        end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesProof> {
        let outbound_lane_data =
            build_messages_data(&self.indexer, &self.outbound, begin, end, block_number).await?;
        let messages_proof = build_darwinia_delivery_proof(
            &self.outbound,
            &self.lane_message_committer,
            &self.chain_message_committer,
            block_number.map(BlockId::from),
        )
        .await?;

        Ok(ReceiveMessagesProof {
            outbound_lane_data,
            messages_proof,
        })
    }

    async fn prepare_for_confirmation(
        &self,
        _begin: u64,
        _end: u64,
        block_number: Option<BlockNumber>,
    ) -> E2EClientResult<ReceiveMessagesDeliveryProof> {
        let block_id = block_number.map(BlockId::from);
        let inbound_lane_data = self.inbound.data(block_id).await?;
        let messages_proof = build_darwinia_confirmation_proof(
            &self.inbound,
            &self.lane_message_committer,
            &self.chain_message_committer,
            block_id,
        )
        .await?;
        Ok(ReceiveMessagesDeliveryProof {
            inbound_lane_data,
            messages_proof,
        })
    }

    fn confirmation_gas_unit(&self) -> E2EClientResult<U256> {
        Ok(U256::from_dec_str("8000000").map_err(|e| E2EClientError::Custom(format!("{}", e)))?)
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
        let execution_payload = block
            .body()
            .execution_payload()
            .map_err(|_| E2EClientError::Custom("No execution payload".into()))?
            .execution_payload_ref();
        let latest_state_root = H256::from(execution_payload.state_root().0);
        if execution_state_root != latest_state_root {
            Ok(None)
        } else {
            let block_number: u64 = execution_payload.block_number();
            Ok(Some(block_number))
        }
    }
}

pub async fn build_darwinia_confirmation_proof(
    inbound: &Inbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> E2EClientResult<Bytes> {
    let (_, lane_pos, _, _) = inbound.get_lane_info(block_id).await?;
    build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await
}

pub async fn build_messages_data(
    indexer: &Thegraph,
    outbound: &Outbound,
    begin: u64,
    end: u64,
    at_block: Option<BlockNumber>,
) -> E2EClientResult<OutboundLaneData> {
    let outbound_data = outbound.data(at_block.map(BlockId::from)).await?;
    let outbound_lane_nonce = outbound
        .outbound_lane_nonce(at_block.map(BlockId::from))
        .await?;
    let (outbound_begin, _outbound_end) = (
        outbound_lane_nonce.latest_received_nonce + 1,
        outbound_lane_nonce.latest_generated_nonce,
    );
    let messages = Vec::from_iter(
        outbound_data.messages[(begin - outbound_begin) as usize..=(end - outbound_begin) as usize]
            .iter()
            .cloned(),
    );

    if (end - begin + 1) as usize != messages.len() {
        return Err(E2EClientError::Custom("Build messages data failed".into()).into());
    }

    let accepted_events = query_message_accepted_events_thegraph(indexer, begin, end).await?;
    let messages: Vec<Message> = std::iter::zip(messages, accepted_events)
        .into_iter()
        .map(|(message, event)| Message {
            encoded_key: message.encoded_key,
            payload: Payload {
                source: event.source,
                target: event.target,
                encoded: event.encoded,
            },
        })
        .collect();

    Ok(OutboundLaneData {
        latest_received_nonce: outbound_data.latest_received_nonce,
        messages,
    })
}

pub async fn build_darwinia_delivery_proof(
    outbound: &Outbound,
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    block_id: Option<BlockId>,
) -> E2EClientResult<Bytes> {
    let (_, lane_pos, _, _) = outbound.get_lane_info().await?;

    build_darwinia_proof(
        lane_message_committer,
        chain_message_committer,
        lane_pos,
        block_id,
    )
    .await
}

async fn build_darwinia_proof(
    lane_message_committer: &LaneMessageCommitter,
    chain_message_committer: &ChainMessageCommitter,
    lane_pos: u32,
    block_id: Option<BlockId>,
) -> E2EClientResult<Bytes> {
    let bridged_chain_pos = lane_message_committer.bridged_chain_position().await?;
    let proof = chain_message_committer
        .prove(bridged_chain_pos, U256::from(lane_pos), block_id)
        .await?
        .into_token();

    Ok(Bytes(encode(&[proof])))
}

pub async fn query_message_accepted_thegraph(
    thegraph_client: &Thegraph,
    nonce: u64,
) -> E2EClientResult<Option<MessageAccepted>> {
    thegraph_client
        .query_message_accepted(nonce)
        .await
        .map_err(|e| E2EClientError::Custom(format!("{}", e)))?
        .map(|x| -> E2EClientResult<MessageAccepted> {
            Ok(MessageAccepted {
                nonce: x.nonce,
                source: Address::from_str(&x.source)
                    .map_err(|e| E2EClientError::Custom(format!("{}", e)))?,
                target: Address::from_str(&x.target)
                    .map_err(|e| E2EClientError::Custom(format!("{}", e)))?,
                encoded: Bytes(
                    hex::decode(&x.encoded[2..])
                        .map_err(|e| E2EClientError::Custom(format!("{}", e)))?,
                ),
                block_number: x.block_number,
            })
        })
        .transpose()
}

pub async fn query_message_accepted_events_thegraph(
    client: &Thegraph,
    begin: u64,
    end: u64,
) -> E2EClientResult<Vec<MessageAccepted>> {
    let logs: Result<Vec<Option<MessageAccepted>>, _> = futures::future::try_join_all(
        (begin..=end).map(|nonce| query_message_accepted_thegraph(client, nonce)),
    )
    .await;
    if let Some(logs) = logs?.into_iter().collect::<Option<Vec<_>>>() {
        Ok(logs)
    } else {
        Err(E2EClientError::Custom(format!(
            "Failed to get message events from {:?} to {:?}",
            begin, end
        ))
        .into())
    }
}
