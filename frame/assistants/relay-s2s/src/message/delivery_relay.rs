use std::ops::RangeInclusive;

use abstract_client_s2s::client::S2SClientRelay;
use abstract_client_s2s::convert::SmartCodecMapper;
use abstract_client_s2s::types::bp_messages::{MessageKey, OutboundLaneData};
use abstract_client_s2s::types::bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use subquery_s2s::types::RelayBlockOrigin;

use crate::error::{RelayError, RelayResult};
use crate::types::MessageRelay;

pub struct DeliveryRunner<SC: S2SClientRelay, TC: S2SClientRelay> {
    message_relay: MessageRelay<SC, TC>,
    last_relayed_nonce: Option<u64>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> DeliveryRunner<SC, TC> {
    pub async fn new(message_relay: MessageRelay<SC, TC>) -> RelayResult<Self> {
        Ok(Self {
            message_relay,
            last_relayed_nonce: None,
        })
    }
}

// defined
impl<SC: S2SClientRelay, TC: S2SClientRelay> DeliveryRunner<SC, TC> {
    async fn source_outbound_lane_data(&self) -> RelayResult<OutboundLaneData> {
        let lane = self.message_relay.lane()?;
        let outbound_lane_data = self
            .message_relay
            .client_source
            .outbound_lanes(lane, None)
            .await?;
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
                    "[delivery-pangolin-to-pangoro] There is already a batch of transactions in progress. \
                    Will wait for the previous batch to complete. last relayed noce is {} and expect to start with {}. \
                    please wait receiving.",
                    last_relayed_nonce,
                    start,
                );
                return Ok(None);
            }
        }

        let inclusive_limit = limit - 1;
        tracing::info!(
            target: "relay-s2s",
            "[delivery-pangolin-to-pangoro] Assemble nonces, start from {} and last generated is {}",
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
            "[delivery-pangolin-to-pangoro] SERVICE RESTARTING..."
        );
        loop {
            match self.run(10).await {
                Ok(last_relayed_nonce) => {
                    if last_relayed_nonce.is_some() {
                        self.last_relayed_nonce = last_relayed_nonce;
                    }
                }
                Err(err) => {
                    tracing::error!(
                        target: "relay-s2s",
                        "[delivery-pangolin-to-pangoro] Failed to delivery message: {:?}",
                        err
                    );
                    // todo: the last_relayed_nonce need return when error happened
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn run(&self, limit: u64) -> RelayResult<Option<u64>> {
        let lane = self.message_relay.lane()?;
        let source_outbound_lane_data = self.source_outbound_lane_data().await?;

        // alias
        let client_source = &self.message_relay.client_source;
        let client_target = &self.message_relay.client_target;
        let subquery_pangolin = &self.message_relay.subquery_source;

        let nonces = match self
            .assemble_nonces(limit, &source_outbound_lane_data)
            .await?
        {
            Some(v) => v,
            None => {
                tracing::info!(
                    target: "relay-s2s",
                    "[delivery-pangolin-to-pangoro] All nonces delivered, nothing to do."
                );
                return Ok(None);
            }
        };
        tracing::info!(
            target: "relay-s2s",
            "[delivery-pangolin-to-pangoro] Assembled nonces {:?}",
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
                    "[delivery-pangolin-to-pangoro] The last nonce({}) isn't storage by indexer",
                    nonces.end(),
                );
                return Ok(None);
            }
        };

        // query last relayed header
        let last_relayed_source_hash_in_target = client_target.best_target_finalized(None).await?;
        let last_relayed_pangolin_block_in_pangoro = client_source
            .block(Some(last_relayed_source_hash_in_target))
            .await?
            .ok_or_else(|| {
                RelayError::Custom(format!(
                    "Failed to query block by [{}] in pangolin",
                    last_relayed_source_hash_in_target
                ))
            })?;

        // compare last nonce block with last relayed header
        let relayed_block_number = last_relayed_pangolin_block_in_pangoro.block.header.number();
        if relayed_block_number < last_relay.block_number {
            tracing::warn!(
                target: "relay-s2s",
                "[delivery-pangolin-to-pangoro] The last nonce({}) at block {} is less then last relayed header {}, \
                please wait header relay.",
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
            .calculate_dispatch_weight(lane, nonces)
            .await?;

        // query last relayed  header
        let proof = client_source
            .read_proof(storage_keys, Some(last_relayed_source_hash_in_target))
            .await?;
        let proof = FromBridgedChainMessagesProof {
            bridged_header_hash: last_relayed_source_hash_in_target,
            storage_proof: proof,
            lane,
            nonces_start: *nonces.start(),
            nonces_end: *nonces.end(),
        };

        let hash = client_target
            .receive_messages_proof(
                client_target.account().account_id().clone(),
                proof,
                (nonces.end() - nonces.start() + 1) as _,
                total_weight,
            )
            .await?;

        tracing::debug!(
            target: "relay-s2s",
            "[delivery-pangolin-to-pangoro] The nonces {:?} in pangolin delivered to pangoro -> {}",
            nonces,
            array_bytes::bytes2hex("0x", hash.0),
        );
        Ok(Some(*nonces.end()))
    }
}
