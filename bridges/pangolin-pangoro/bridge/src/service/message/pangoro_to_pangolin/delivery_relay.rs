use std::ops::RangeInclusive;

use client_pangolin::types::runtime_types::bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use client_pangoro::subxt_runtime::api::bridge_pangolin_messages::storage::{
    OutboundLanes, OutboundMessages,
};
use client_pangoro::types::runtime_types as pangoro_runtime_types;
use client_pangoro::types::runtime_types::bp_messages::{MessageKey, OutboundLaneData};
use subquery_s2s::types::RelayBlockOrigin;
use subxt::storage::StorageKeyPrefix;
use subxt::StorageEntry;

use support_common::error::BridgerError;

use crate::service::message::types::MessageRelay;

/// Message payload for This -> Bridged chain messages.
type FromThisChainMessagePayload = pangoro_runtime_types::bp_message_dispatch::MessagePayload<
    sp_core::crypto::AccountId32,
    pangoro_runtime_types::sp_runtime::MultiSigner,
    pangoro_runtime_types::sp_runtime::MultiSignature,
    Vec<u8>,
>;

pub struct DeliveryRunner {
    message_relay: MessageRelay,
    last_relayed_nonce: Option<u64>,
}

impl DeliveryRunner {
    pub async fn new() -> color_eyre::Result<Self> {
        let message_relay = MessageRelay::new().await?;
        Ok(Self {
            message_relay,
            last_relayed_nonce: None,
        })
    }
}

// defined
impl DeliveryRunner {
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

    async fn assemble_nonces(
        &self,
        limit: u64,
        outbound_lane_data: &OutboundLaneData,
    ) -> color_eyre::Result<Option<RangeInclusive<u64>>> {
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
                    target: "pangolin-pangoro",
                    "[delivery-pangoro-to-pangolin] There is already a batch of transactions in progress. \
                    Will wait for the previous batch to complete. last relayed noce is {} and expect to start with {}",
                    last_relayed_nonce,
                    start,
                );
                return Ok(None);
            }
        }

        let inclusive_limit = limit - 1;
        tracing::info!(
            target: "pangolin-pangoro",
            "[delivery-pangoro-to-pangolin] Assemble nonces, start from {} and last generated is {}",
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

impl DeliveryRunner {
    pub async fn start(&mut self) -> color_eyre::Result<()> {
        tracing::info!(
            target: "pangolin-pangoro",
            "[delivery-pangoro-to-pangolin] SERVICE RESTARTING..."
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
                        target: "pangolin-pangoro",
                        "[delivery-pangoro-to-pangolin] Failed to delivery message: {:?}",
                        err
                    );
                    self.message_relay = MessageRelay::new().await?;
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn run(&self, limit: u64) -> color_eyre::Result<Option<u64>> {
        let lane = self.message_relay.lane()?;
        let source_outbound_lane_data = self.source_outbound_lane_data().await?;

        // alias
        let client_pangoro = &self.message_relay.client_pangoro;
        let client_pangolin = &self.message_relay.client_pangolin;
        let subquery_pangoro = &self.message_relay.subquery_pangoro;

        let nonces = match self
            .assemble_nonces(limit, &source_outbound_lane_data)
            .await?
        {
            Some(v) => v,
            None => {
                tracing::info!(
                    target: "pangolin-pangoro",
                    "[delivery-pangoro-to-pangolin] All nonces delivered, nothing to do."
                );
                return Ok(None);
            }
        };
        tracing::info!(
            target: "pangolin-pangoro",
            "[delivery-pangoro-to-pangolin] Assembled nonces {:?}",
            nonces,
        );

        // query last nonce block information
        let last_relay = match subquery_pangoro
            .query_need_relay(RelayBlockOrigin::BridgePangolin, lane.0, *nonces.end())
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "pangolin-pangoro",
                    "[delivery-pangoro-to-pangolin] The last nonce({}) isn't storage by indexer",
                    nonces.end(),
                );
                return Ok(None);
            }
        };

        // query last relayed header
        let last_relayed_pangoro_hash_in_pangolin = client_pangolin
            .runtime()
            .storage()
            .bridge_pangoro_grandpa()
            .best_finalized(None)
            .await?;
        let last_relayed_pangoro_block_in_pangolin = client_pangoro
            .subxt()
            .rpc()
            .block(Some(last_relayed_pangoro_hash_in_pangolin))
            .await?
            .ok_or_else(|| {
                BridgerError::Custom(format!(
                    "Failed to query block by [{}] in pangoro",
                    last_relayed_pangoro_hash_in_pangolin
                ))
            })?;

        // compare last nonce block with last relayed header
        let relayed_block_number = last_relayed_pangoro_block_in_pangolin.block.header.number;
        if relayed_block_number < last_relay.block_number {
            tracing::warn!(
                target: "pangolin-pangoro",
                "[delivery-pangoro-to-pangolin] The last nonce({}) at block {} is less then last relayed header {}, \
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
            let prefix = StorageKeyPrefix::new::<OutboundMessages>();
            let message_key = OutboundMessages(MessageKey {
                lane_id: lane.0,
                nonce: message_nonce,
            })
            .key()
            .final_key(prefix);
            storage_keys.push(message_key);
            message_nonce += 1;
        }

        //- query inbound land data
        let target_inbound_lane_data = client_pangolin
            .runtime()
            .storage()
            .bridge_pangoro_messages()
            .inbound_lanes(lane.0, None)
            .await?;
        let outbound_state_proof_required = target_inbound_lane_data.last_confirmed_nonce
            < source_outbound_lane_data.latest_received_nonce;
        if outbound_state_proof_required {
            storage_keys.push(
                OutboundLanes(lane.0)
                    .key()
                    .final_key(StorageKeyPrefix::new::<OutboundLanes>()),
            );
        }

        // query last relayed header
        let read_proof = client_pangoro
            .subxt()
            .rpc()
            .read_proof(storage_keys, Some(last_relayed_pangoro_hash_in_pangolin))
            .await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        let proof = FromBridgedChainMessagesProof {
            bridged_header_hash: last_relayed_pangoro_hash_in_pangolin,
            storage_proof: proof,
            lane: lane.0,
            nonces_start: *nonces.start(),
            nonces_end: *nonces.end(),
        };

        // fill delivery data
        let mut total_weight = 0u64;
        for message_nonce in nonces.clone() {
            let message_data = client_pangoro
                .runtime()
                .storage()
                .bridge_pangolin_messages()
                .outbound_messages(
                    MessageKey {
                        lane_id: lane.0,
                        nonce: message_nonce,
                    },
                    None,
                )
                .await?
                .ok_or_else(|| {
                    BridgerError::Custom(format!(
                        "Can not read message data by nonce {} in pangoro",
                        message_nonce
                    ))
                })?;
            let decoded_payload: FromThisChainMessagePayload =
                codec::Decode::decode(&mut &message_data.payload[..])?;
            total_weight += decoded_payload.weight;
        }

        let hash = client_pangolin
            .runtime()
            .tx()
            .bridge_pangoro_messages()
            .receive_messages_proof(
                client_pangolin.account().account_id().clone(),
                proof,
                (nonces.end() - nonces.start() + 1) as _,
                total_weight,
            )
            .sign_and_submit(client_pangolin.account().signer())
            .await?;

        tracing::debug!(
            target: "pangolin-pangoro",
            "[delivery-pangoro-to-pangolin] The nonces {:?} in pangoro delivered to pangolin -> {}",
            nonces,
            array_bytes::bytes2hex("0x", hash.0),
        );
        Ok(Some(*nonces.end()))
    }
}
