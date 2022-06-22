use std::ops::RangeInclusive;

use abstract_bridge_s2s::client::S2SClientRelay;
use abstract_bridge_s2s::convert::SmartCodecMapper;
use abstract_bridge_s2s::types::bp_messages::OutboundLaneData;
use abstract_bridge_s2s::types::bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use sp_runtime::traits::Header;
use subquery_s2s::types::RelayBlockOrigin;

use crate::error::{RelayError, RelayResult};
use crate::helpers;
use crate::types::{MessageInput, M_DELIVERY};

pub struct DeliveryRunner<SC: S2SClientRelay, TC: S2SClientRelay> {
    input: MessageInput<SC, TC>,
    last_relayed_nonce: Option<u64>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> DeliveryRunner<SC, TC> {
    pub fn new(input: MessageInput<SC, TC>) -> Self {
        Self {
            input,
            last_relayed_nonce: None,
        }
    }
}

// defined
impl<SC: S2SClientRelay, TC: S2SClientRelay> DeliveryRunner<SC, TC> {
    async fn source_outbound_lane_data(&self) -> RelayResult<OutboundLaneData> {
        let lane = self.input.lane()?;
        let outbound_lane_data = self.input.client_source.outbound_lanes(lane, None).await?;
        Ok(outbound_lane_data)
    }

    async fn assemble_nonces(
        &self,
        limit: u64,
        outbound_lane_data: &OutboundLaneData,
    ) -> RelayResult<Option<RangeInclusive<u64>>> {
        let (latest_confirmed_nonce, latest_generated_nonce) = (
            outbound_lane_data.latest_received_nonce,
            outbound_lane_data.latest_generated_nonce,
        );
        if latest_confirmed_nonce == latest_generated_nonce {
            return Ok(None);
        }

        // assemble nonce range
        let start: u64 = latest_confirmed_nonce + 1;
        if let Some(last_relayed_nonce) = self.last_relayed_nonce {
            if last_relayed_nonce >= start {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} have a batches of transactions in progress. \
                    waiting for this batches to complete. last relayed noce is {} and expect to start with {}. \
                    please wait receiving.",
                    helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
                    last_relayed_nonce,
                    start,
                );
                return Ok(None);
            }
        }

        let inclusive_limit = limit - 1;
        tracing::trace!(
            target: "relay-s2s",
            "{} assemble nonces, start from {} and last generated is {}",
            helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
            start,
            latest_generated_nonce,
        );
        let end: u64 = if latest_generated_nonce - start > inclusive_limit {
            start + inclusive_limit
        } else {
            latest_generated_nonce
        };
        let nonces = start..=end;
        Ok(Some(nonces))
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> DeliveryRunner<SC, TC> {
    pub async fn start(&mut self) -> RelayResult<()> {
        tracing::info!(
            target: "relay-s2s",
            "{} SERVICE RESTARTING...",
            helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
        );
        loop {
            if let Ok(last_relayed_nonce) = self.run(10).await {
                if last_relayed_nonce.is_some() {
                    self.last_relayed_nonce = last_relayed_nonce;
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    async fn run(&self, limit: u64) -> RelayResult<Option<u64>> {
        let lane = self.input.lane()?;
        let source_outbound_lane_data = self.source_outbound_lane_data().await?;

        // alias
        let client_source = &self.input.client_source;
        let client_target = &self.input.client_target;
        let subquery_pangolin = &self.input.subquery_source;

        let nonces = match self
            .assemble_nonces(limit, &source_outbound_lane_data)
            .await?
        {
            Some(v) => v,
            None => {
                tracing::info!(
                    target: "relay-s2s",
                    "{} all nonces delivered, nothing to do.",
                    helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
                );
                return Ok(None);
            }
        };
        tracing::info!(
            target: "relay-s2s",
            "{} assembled nonces {:?}",
            helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
            nonces,
        );

        // query last nonce block information
        let last_relay = match subquery_pangolin
            .query_need_relay(RelayBlockOrigin::BridgePangoro, lane, *nonces.end())
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} the last nonce({}) isn't storage by indexer",
                    helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
                    nonces.end(),
                );
                return Ok(None);
            }
        };

        // query last relayed header
        let last_relayed_source_hash_in_target = client_target.best_target_finalized(None).await?;
        let expected_source_hash = SmartCodecMapper::map_to(&last_relayed_source_hash_in_target)?;
        let last_relayed_pangolin_block_in_pangoro = client_source
            .block(Some(expected_source_hash))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!(
                    "Failed to query block by [{}] in pangolin",
                    array_bytes::bytes2hex("0x", expected_source_hash),
                ))
            })?;

        // compare last nonce block with last relayed header
        let relayed_block_number = last_relayed_pangolin_block_in_pangoro.block.header.number();
        let relayed_block_number: u32 = SmartCodecMapper::map_to(relayed_block_number)?;
        if relayed_block_number < last_relay.block_number {
            tracing::warn!(
                target: "relay-s2s",
                "{} the last nonce({}) at block {} is less then last relayed header {}, please wait header relay.",
                helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
                nonces.end(),
                last_relay.block_number,
                relayed_block_number,
            );
            return Ok(None);
        }

        // read proof
        let mut storage_keys = Vec::with_capacity((nonces.end() - nonces.start()) as usize + 1);
        let mut message_nonce = *nonces.start();
        while message_nonce <= *nonces.end() {
            let message_key = client_source.gen_outbound_messages_storage_key(lane, message_nonce);
            storage_keys.push(message_key);
            message_nonce += 1;
        }

        //- query inbound land data
        let target_inbound_lane_data = client_target.inbound_lanes(lane, None).await?;
        let outbound_state_proof_required = target_inbound_lane_data.last_confirmed_nonce
            < source_outbound_lane_data.latest_received_nonce;
        if outbound_state_proof_required {
            storage_keys.push(client_source.gen_outbound_lanes_storage_key(lane));
        }

        // fill delivery data
        let total_weight = client_source
            .calculate_dispatch_weight(lane, nonces.clone())
            .await?;

        // query last relayed  header
        let proof = client_source
            .read_proof(storage_keys, Some(expected_source_hash))
            .await?;
        let proof = FromBridgedChainMessagesProof {
            bridged_header_hash: expected_source_hash,
            storage_proof: proof,
            lane,
            nonces_start: *nonces.start(),
            nonces_end: *nonces.end(),
        };

        let expected_proof = SmartCodecMapper::map_to(&proof)?;
        let relayer_account_source_chain = self.input.relayer_account.clone();
        let expected_relayer_id = SmartCodecMapper::map_to(&relayer_account_source_chain)?;
        let hash = client_target
            .receive_messages_proof(
                expected_relayer_id,
                expected_proof,
                (nonces.end() - nonces.start() + 1) as _,
                total_weight,
            )
            .await?;

        tracing::debug!(
            target: "relay-s2s",
            "{} the nonces {:?} in pangolin delivered to pangoro -> {}",
            helpers::log_prefix(M_DELIVERY, SC::CHAIN, TC::CHAIN),
            nonces,
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(Some(*nonces.end()))
    }
}
