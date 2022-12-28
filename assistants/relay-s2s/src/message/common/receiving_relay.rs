use bridge_s2s_traits::client::S2SClientRelay;
use bridge_s2s_traits::types::bp_messages::{OutboundLaneData, UnrewardedRelayersState};
use bridge_s2s_traits::types::bp_runtime::Chain;
use bridge_s2s_traits::types::bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof;

use support_toolkit::{convert::SmartCodecMapper, logk};

use crate::error::RelayResult;
use crate::keepstate;
use crate::special::DifferentClientApi;
use crate::types::{LaneId, MessageReceivingInput, M_RECEIVING};

pub struct CommonReceivingRunner<SC: S2SClientRelay, TC: S2SClientRelay, DC: DifferentClientApi<SC>>
{
    different: DC,
    input: MessageReceivingInput<SC, TC>,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay, DC: DifferentClientApi<SC>>
    CommonReceivingRunner<SC, TC, DC>
{
    pub fn new(message_relay: MessageReceivingInput<SC, TC>, different: DC) -> Self {
        Self {
            different,
            input: message_relay,
        }
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay, DC: DifferentClientApi<SC>>
    CommonReceivingRunner<SC, TC, DC>
{
    async fn source_outbound_lane_data(&self, lane: LaneId) -> RelayResult<OutboundLaneData> {
        let outbound_lane_data = self.input.client_source.outbound_lanes(lane, None).await?;
        Ok(outbound_lane_data)
    }

    async fn target_unrewarded_relayers_state(
        &self,
        lane: LaneId,
        at_block: <TC::Chain as Chain>::Hash,
        source_outbound_lane_data: &OutboundLaneData,
    ) -> RelayResult<Option<(u64, UnrewardedRelayersState)>> {
        let block_hex = array_bytes::bytes2hex("0x", at_block);
        let inbound_lane_data = self
            .input
            .client_target
            .inbound_lanes(lane, Some(at_block))
            .await?;
        let last_delivered_nonce = inbound_lane_data.last_delivered_nonce();
        let max_confirm_end_at_target = inbound_lane_data
            .relayers
            .iter()
            .map(|item| item.messages.end)
            .max()
            .unwrap_or(0u64);
        if max_confirm_end_at_target == source_outbound_lane_data.latest_received_nonce {
            tracing::info!(
                target: "relay-s2s",
                "{} the last dispatched nonce({}) at target({}) is same with last received nonce({}) at source. nothing to do.",
                logk::prefix_with_bridge(M_RECEIVING, SC::CHAIN, TC::CHAIN),
                max_confirm_end_at_target,
                block_hex,
                source_outbound_lane_data.latest_received_nonce,
            );
            return Ok(None);
        }
        if let Some(last_relayed_nonce) = keepstate::get_last_receiving_relayed_nonce(TC::CHAIN) {
            if last_relayed_nonce >= max_confirm_end_at_target {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} the nonce({}) is being processed. please waiting for the processing to finish.",
                    logk::prefix_with_bridge(M_RECEIVING, SC::CHAIN, TC::CHAIN),
                    max_confirm_end_at_target,
                );
                return Ok(None);
            }
        }
        let relayers = inbound_lane_data.relayers;
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
            tracing::info!(
                target: "relay-s2s",
                "{} not have unrewarded message. nothing to do.",
                logk::prefix_with_bridge(M_RECEIVING, SC::CHAIN, TC::CHAIN),
            );
            return Ok(None);
        }
        Ok(Some((
            max_confirm_end_at_target,
            UnrewardedRelayersState {
                unrewarded_relayer_entries: relayers.len() as _,
                messages_in_oldest_entry: relayers
                    .front()
                    .map(|entry| 1 + entry.messages.end - entry.messages.begin)
                    .unwrap_or(u64::MAX),
                total_messages: total_unrewarded_messages.expect("Unreachable"),
                last_delivered_nonce,
            },
        )))
    }
}

impl<SC: S2SClientRelay, TC: S2SClientRelay, DC: DifferentClientApi<SC>>
    CommonReceivingRunner<SC, TC, DC>
{
    pub async fn start(&self) -> RelayResult<()> {
        tracing::info!(
            target: "relay-s2s",
            "{} SERVICE RESTARTING...",
            logk::prefix_with_bridge(M_RECEIVING, SC::CHAIN, TC::CHAIN),
        );
        loop {
            for lane in &self.input.lanes {
                let last_relayed_nonce = self.run(*lane).await?;
                if last_relayed_nonce.is_some() {
                    keepstate::set_last_receiving_relayed_nonce(
                        TC::CHAIN,
                        last_relayed_nonce.expect("Unreachable"),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    async fn run(&self, lane: LaneId) -> RelayResult<Option<u64>> {
        // alias
        let client_source = &self.input.client_source;
        let client_target = &self.input.client_target;

        let source_outbound_lane_data = self.source_outbound_lane_data(lane).await?;
        if source_outbound_lane_data.latest_received_nonce
            == source_outbound_lane_data.latest_generated_nonce
        {
            tracing::debug!(
                target: "relay-s2s",
                "{} all nonces received, nothing to do.",
                logk::prefix_with_bridge_and_others(
                    M_RECEIVING,
                    SC::CHAIN,
                    TC::CHAIN,
                    vec![array_bytes::bytes2hex("0x", &lane),],
                ),
            );
            return Ok(None);
        }

        // query last relayed header (from source chain)
        let last_relayed_target_block_in_source =
            match self.different.best_target_finalized(None).await? {
                Some(v) => v,
                None => {
                    tracing::warn!(
                        target: "relay-s2s",
                        "{} the bridge not initialized, please init first.",
                        logk::prefix_with_bridge(
                            M_RECEIVING,
                            SC::CHAIN,
                            TC::CHAIN,
                        ),
                    );
                    return Ok(None);
                }
            };
        let expected_target_hash =
            SmartCodecMapper::map_to(&last_relayed_target_block_in_source.1)?;

        // assemble unrewarded relayers state
        let (max_confirmed_nonce_at_target, relayers_state) = match self
            .target_unrewarded_relayers_state(
                lane,
                expected_target_hash,
                &source_outbound_lane_data,
            )
            .await?
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    target: "relay-s2s",
                    "{} no unrewarded relayers state found by {}",
                    logk::prefix_with_bridge_and_others(
                        M_RECEIVING,
                        SC::CHAIN,
                        TC::CHAIN,
                        vec![array_bytes::bytes2hex("0x", &lane),],
                    ),
                    TC::CHAIN,
                );
                return Ok(None);
            }
        };

        // read proof
        let inbound_data_key = client_target.gen_inbound_lanes_storage_key(lane);
        let proof = client_target
            .read_proof(vec![inbound_data_key], Some(expected_target_hash))
            .await?;
        let proof = FromBridgedChainMessagesDeliveryProof {
            bridged_header_hash: last_relayed_target_block_in_source.1,
            storage_proof: proof,
            lane,
        };

        // send proof
        let hash = client_source
            .receive_messages_delivery_proof(proof, relayers_state)
            .await?;

        tracing::info!(
            target: "relay-s2s",
            "{} receiving extensics sent successful: {}",
            logk::prefix_with_bridge_and_others(
                M_RECEIVING,
                SC::CHAIN,
                TC::CHAIN,
                vec![array_bytes::bytes2hex("0x", &lane),],
            ),
            array_bytes::bytes2hex("0x", hash),
        );
        Ok(Some(max_confirmed_nonce_at_target))
    }
}
