use std::ops::RangeInclusive;

use bridge_s2s_traits::client::{
    S2SClientBase, S2SClientGeneric, S2SClientRelay, S2SParaBridgeClientSolochain,
};
use bridge_s2s_traits::error::{S2SClientError, S2SClientResult};
use bridge_s2s_traits::types::{
    bp_header_chain, bp_messages, bp_runtime::Chain, bridge_runtime_common,
};
use sp_runtime::AccountId32;
use subxt::sp_core::storage::StorageKey;
use subxt::storage::StorageKeyPrefix;
use subxt::StorageEntry;

use support_toolkit::convert::SmartCodecMapper;

use crate::client::CrabClient;
use crate::error::ClientError;
use crate::subxt_runtime::api::bridge_darwinia_messages::storage::{
    InboundLanes, OutboundLanes, OutboundMessages,
};

type BundleMessageKey = crate::types::runtime_types::bp_messages::MessageKey;

/// Message payload for This -> Bridged chain messages.
type FromThisChainMessagePayload = crate::types::runtime_types::bp_message_dispatch::MessagePayload<
    sp_core::crypto::AccountId32,
    crate::types::runtime_types::sp_runtime::MultiSigner,
    crate::types::runtime_types::sp_runtime::MultiSignature,
    Vec<u8>,
>;

#[async_trait::async_trait]
impl S2SClientRelay for CrabClient {
    fn gen_outbound_messages_storage_key(&self, lane: [u8; 4], message_nonce: u64) -> StorageKey {
        let prefix = StorageKeyPrefix::new::<OutboundMessages>();
        OutboundMessages(BundleMessageKey {
            lane_id: lane,
            nonce: message_nonce,
        })
        .key()
        .final_key(prefix)
    }

    fn gen_outbound_lanes_storage_key(&self, lane: [u8; 4]) -> StorageKey {
        OutboundLanes(lane)
            .key()
            .final_key(StorageKeyPrefix::new::<OutboundLanes>())
    }

    fn gen_inbound_lanes_storage_key(&self, lane: [u8; 4]) -> StorageKey {
        InboundLanes(lane)
            .key()
            .final_key(StorageKeyPrefix::new::<InboundLanes>())
    }

    async fn calculate_dispatch_weight(
        &self,
        lane: [u8; 4],
        nonces: RangeInclusive<u64>,
    ) -> S2SClientResult<u64> {
        let mut total_weight = 0u64;
        for message_nonce in nonces {
            let message_data = self
                .outbound_messages(
                    bp_messages::MessageKey {
                        lane_id: lane,
                        nonce: message_nonce,
                    },
                    None,
                )
                .await?
                .ok_or_else(|| {
                    ClientError::Custom(format!(
                        "Can not read message data by nonce {} in darwinia",
                        message_nonce
                    ))
                })?;
            let decoded_payload: FromThisChainMessagePayload =
                codec::Decode::decode(&mut &message_data.payload[..])?;
            total_weight += decoded_payload.weight;
        }
        Ok(total_weight)
    }

    async fn best_target_finalized(
        &self,
        at_block: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        Ok(self
            .runtime()
            .storage()
            .bridge_darwinia_grandpa()
            .best_finalized(at_block)
            .await?)
    }

    async fn initialize(
        &self,
        initialization_data: <Self as S2SClientGeneric>::InitializationData,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .bridge_darwinia_grandpa()
            .initialize(initialization_data)
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }

    async fn submit_finality_proof(
        &self,
        finality_target: <Self::Chain as Chain>::Header,
        justification: bp_header_chain::justification::GrandpaJustification<
            <Self::Chain as Chain>::Header,
        >,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let expected_target = SmartCodecMapper::map_to(&finality_target)?;
        let expected_justification = SmartCodecMapper::map_to(&justification)?;
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .bridge_darwinia_grandpa()
            .submit_finality_proof(expected_target, expected_justification)
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }

    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<bp_messages::OutboundLaneData> {
        let outbound_lane_data = self
            .runtime()
            .storage()
            .bridge_darwinia_messages()
            .outbound_lanes(lane, hash)
            .await?;
        let expected = SmartCodecMapper::map_to(&outbound_lane_data)?;
        Ok(expected)
    }

    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<bp_messages::InboundLaneData<sp_core::crypto::AccountId32>> {
        let inbound_lane_data = self
            .runtime()
            .storage()
            .bridge_darwinia_messages()
            .inbound_lanes(lane, hash)
            .await?;
        let expected = SmartCodecMapper::map_to(&inbound_lane_data)?;
        Ok(expected)
    }

    async fn outbound_messages(
        &self,
        message_key: bp_messages::MessageKey,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<bp_messages::MessageData<u128>>> {
        let expected_message_key = SmartCodecMapper::map_to(&message_key)?;
        match self
            .runtime()
            .storage()
            .bridge_darwinia_messages()
            .outbound_messages(expected_message_key, hash)
            .await?
        {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }

    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: AccountId32,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<
            <Self::Chain as Chain>::Hash,
        >,
        messages_count: u32,
        dispatch_weight: u64,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let expected_proof = SmartCodecMapper::map_to(&proof)?;
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .bridge_darwinia_messages()
            .receive_messages_proof(
                relayer_id_at_bridged_chain,
                expected_proof,
                messages_count,
                dispatch_weight,
            )
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }

    async fn receive_messages_delivery_proof(
        &self,
        proof: bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
            <Self::Chain as Chain>::Hash,
        >,
        relayers_state: bp_messages::UnrewardedRelayersState,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let expected_proof = SmartCodecMapper::map_to(&proof)?;
        let expected_relayers_state = SmartCodecMapper::map_to(&relayers_state)?;
        let runtime = self.runtime();
        let track = runtime
            .tx()
            .bridge_darwinia_messages()
            .receive_messages_delivery_proof(expected_proof, expected_relayers_state)
            .sign_and_submit_then_watch(self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transactioni failed {}: {:?}",
                <Self as S2SClientBase>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }
}
