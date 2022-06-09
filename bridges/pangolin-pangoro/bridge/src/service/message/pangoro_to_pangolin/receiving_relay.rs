use client_pangolin::subxt_runtime::api::bridge_pangoro_messages::storage::InboundLanes;
use client_pangoro::types::runtime_types::bp_messages::{
    OutboundLaneData, UnrewardedRelayersState,
};
use client_pangoro::types::runtime_types::bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof;
use std::collections::VecDeque;
use subxt::storage::StorageKeyPrefix;
use subxt::StorageEntry;

use crate::service::message::types::MessageRelay;

pub struct ReceivingRunner {
    message_relay: MessageRelay,
}

impl ReceivingRunner {
    pub async fn new() -> color_eyre::Result<Self> {
        let message_relay = MessageRelay::new().await?;
        Ok(Self { message_relay })
    }
}

impl ReceivingRunner {
    async fn source_outbound_lane_data(&self) -> color_eyre::Result<OutboundLaneData> {
        let lane = self.message_relay.lane()?;
        let outbound_lane_data = self
            .message_relay
            .client_pangoro
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_lanes(lane.0, None)
            .await?;
        Ok(outbound_lane_data)
    }

    async fn target_unrewarded_relayers_state(
        &self,
        at_block: sp_core::H256,
        source_outbound_lane_data: &OutboundLaneData,
    ) -> color_eyre::Result<Option<UnrewardedRelayersState>> {
        let lane = self.message_relay.lane()?;
        let inbound_lane_data = self
            .message_relay
            .client_pangolin
            .runtime()
            .storage()
            .bridge_pangoro_messages()
            .inbound_lanes(lane.0, Some(at_block))
            .await?;
        let max_confirm_end_at_target = inbound_lane_data
            .relayers
            .iter()
            .map(|item| item.messages.end)
            .max()
            .unwrap_or(0u64);
        if max_confirm_end_at_target == source_outbound_lane_data.latest_received_nonce {
            return Ok(None);
        }
        let relayers = VecDeque::from_iter(inbound_lane_data.relayers.as_slice());
        let total_unrewarded_messages = match (relayers.front(), relayers.back()) {
            (Some(front), Some(back)) => {
                if back.messages.end < front.messages.begin {
                    Some(0)
                } else {
                    let difference = back.messages.end - front.messages.begin;
                    Some(difference + 1)
                }
            }
            _ => Some(0),
        };
        if total_unrewarded_messages.is_none() {
            return Ok(None);
        }
        Ok(Some(UnrewardedRelayersState {
            unrewarded_relayer_entries: relayers.len() as _,
            messages_in_oldest_entry: relayers
                .front()
                .map(|entry| 1 + entry.messages.end - entry.messages.begin)
                .unwrap_or(0),
            total_messages: total_unrewarded_messages.expect("Unreachable"),
        }))
    }
}

impl ReceivingRunner {
    pub async fn start(&mut self) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-pangoro",
            "[receiving-pangoro-to-pangolin] SERVICE RESTARTING..."
        );
        loop {
            match self.run().await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[receiving-pangoro-to-pangolin] Failed to receiving message: {:?}",
                        err
                    );
                    self.message_relay = MessageRelay::new().await?;
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn run(&self) -> color_eyre::Result<()> {
        let lane = self.message_relay.lane()?;

        // alias
        let client_pangoro = &self.message_relay.client_pangoro;
        let client_pangolin = &self.message_relay.client_pangolin;

        let source_outbound_lane_data = self.source_outbound_lane_data().await?;
        if source_outbound_lane_data.latest_received_nonce
            == source_outbound_lane_data.latest_generated_nonce
        {
            tracing::info!(
                target: "pangolin-pangoro",
                "[receiving-pangoro-to-pangolin] All nonces received, nothing to do.",
            );
            return Ok(());
        }

        // query last relayed header
        let last_relayed_pangolin_hash_in_pangoro = client_pangoro
            .runtime()
            .storage()
            .bridge_pangolin_grandpa()
            .best_finalized(None)
            .await?;

        // assemble unrewarded relayers state
        let relayers_state = match self
            .target_unrewarded_relayers_state(
                last_relayed_pangolin_hash_in_pangoro,
                &source_outbound_lane_data,
            )
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "pangolin-pangoro",
                    "[receiving-pangoro-to-pangolin] No unrewarded relayers state found by pangolin",
                );
                return Ok(());
            }
        };

        // read proof
        let inbound_data_key = InboundLanes(lane.0)
            .key()
            .final_key(StorageKeyPrefix::new::<InboundLanes>());
        let read_proof = client_pangolin
            .subxt()
            .rpc()
            .read_proof(
                vec![inbound_data_key],
                Some(last_relayed_pangolin_hash_in_pangoro),
            )
            .await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        let proof = FromBridgedChainMessagesDeliveryProof {
            bridged_header_hash: last_relayed_pangolin_hash_in_pangoro,
            storage_proof: proof,
            lane: lane.0,
        };

        // send proof
        let hash = client_pangoro
            .runtime()
            .tx()
            .bridge_pangolin_messages()
            .receive_messages_delivery_proof(proof, relayers_state)
            .sign_and_submit(client_pangoro.account().signer())
            .await?;

        tracing::debug!(
            target: "pangolin-pangoro",
            "[receiving-pangoro-to-pangolin] receiving extensics sent successful: {}",
            array_bytes::bytes2hex("0x", hash.0),
        );
        Ok(())
    }
}
