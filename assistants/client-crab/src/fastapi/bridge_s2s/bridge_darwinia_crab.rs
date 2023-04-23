use std::ops::RangeInclusive;

use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay, S2SParaBridgeClientSolochain};
use bridge_s2s_traits::error::{S2SClientError, S2SClientResult};
use bridge_s2s_traits::types::bp_messages::Weight;
use bridge_s2s_traits::types::{
    bp_header_chain, bp_messages, bp_runtime::Chain, bridge_runtime_common,
};
use client_common_traits::ClientCommon;

use support_toolkit::convert::SmartCodecMapper;

use crate::client::CrabClient;
use crate::error::ClientError;

/// Message payload for This -> Bridged chain messages.
type FromThisChainMessagePayload = crate::types::runtime_types::bp_message_dispatch::MessagePayload<
    crate::types::runtime_types::account::AccountId20,
    crate::types::runtime_types::account::EthereumSigner,
    crate::types::runtime_types::account::EthereumSignature,
    Vec<u8>,
>;

#[async_trait::async_trait]
impl S2SClientRelay for CrabClient {
    fn gen_outbound_messages_storage_key(&self, lane: [u8; 4], message_nonce: u64) -> Vec<u8> {
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .outbound_messages(
                &crate::subxt_runtime::api::runtime_types::bp_messages::MessageKey {
                    lane_id: lane,
                    nonce: message_nonce,
                },
            );
        address.to_bytes()
    }

    fn gen_outbound_lanes_storage_key(&self, lane: [u8; 4]) -> Vec<u8> {
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .outbound_lanes(&lane);
        address.to_bytes()
    }

    fn gen_inbound_lanes_storage_key(&self, lane: [u8; 4]) -> Vec<u8> {
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .inbound_lanes(&lane);
        address.to_bytes()
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
                        "Can not read message data by nonce {} in crab",
                        message_nonce
                    ))
                })?;
            let decoded_payload: FromThisChainMessagePayload =
                codec::Decode::decode(&mut &message_data.payload[..])?;
            total_weight += decoded_payload.weight.ref_time;
        }
        Ok(total_weight)
    }

    async fn best_target_finalized(
        &self,
        at_block: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<
        Option<(
            <Self::Chain as Chain>::BlockNumber,
            <Self::Chain as Chain>::Hash,
        )>,
    > {
        let address = crate::subxt_runtime::api::storage()
            .bridge_polkadot_grandpa()
            .best_finalized();
        match self.subxt().storage().fetch(&address, at_block).await? {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }

    async fn initialize(
        &self,
        initialization_data: <Self as S2SClientGeneric>::InitializationData,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let call = crate::subxt_runtime::api::tx()
            .bridge_polkadot_grandpa()
            .initialize(initialization_data);
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch_default(&call, self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transaction failed {}: {:?}",
                <Self as ClientCommon>::CHAIN,
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

        let call = crate::subxt_runtime::api::tx()
            .bridge_polkadot_grandpa()
            .submit_finality_proof(expected_target, expected_justification);
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch_default(&call, self.account().signer())
            .await?;

        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transaction failed {}: {:?}",
                <Self as ClientCommon>::CHAIN,
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
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .outbound_lanes(&lane);
        let outbound_lane_data = self
            .subxt()
            .storage()
            .fetch_or_default(&address, hash)
            .await?;
        let expected = SmartCodecMapper::map_to(&outbound_lane_data)?;
        Ok(expected)
    }

    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<bp_messages::InboundLaneData<<Self::Chain as Chain>::AccountId>> {
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .inbound_lanes(&lane);
        let inbound_lane_data = self
            .subxt()
            .storage()
            .fetch_or_default(&address, hash)
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
        let address = crate::subxt_runtime::api::storage()
            .bridge_darwinia_messages()
            .outbound_messages(&expected_message_key);
        match self.subxt().storage().fetch(&address, hash).await? {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }

    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: <Self::Chain as Chain>::AccountId,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<
            <Self::Chain as Chain>::Hash,
        >,
        messages_count: u32,
        dispatch_weight: Weight,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let relayer_id_at_bridged_chain = SmartCodecMapper::map_to(&relayer_id_at_bridged_chain)?;
        let expected_proof = SmartCodecMapper::map_to(&proof)?;
        let expected_dispatch_weight = SmartCodecMapper::map_to(&dispatch_weight)?;
        let call = crate::subxt_runtime::api::tx()
            .bridge_darwinia_messages()
            .receive_messages_proof(
                relayer_id_at_bridged_chain,
                expected_proof,
                messages_count,
                expected_dispatch_weight,
            );
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch_default(&call, self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transaction failed {}: {:?}",
                <Self as ClientCommon>::CHAIN,
                e,
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
        let call = crate::subxt_runtime::api::tx()
            .bridge_darwinia_messages()
            .receive_messages_delivery_proof(expected_proof, expected_relayers_state);
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch_default(&call, self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transaction failed {}: {:?}",
                <Self as ClientCommon>::CHAIN,
                e,
            ))
        })?;
        Ok(events.extrinsic_hash())
    }
}

#[async_trait::async_trait]
impl S2SParaBridgeClientSolochain for CrabClient {
    async fn best_para_heads(
        &self,
        para_id: bridge_s2s_traits::types::ParaId,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<bridge_s2s_traits::types::ParaInfo>> {
        let expected_para_id = SmartCodecMapper::map_to(&para_id)?;
        let address = crate::subxt_runtime::api::storage()
            .bridge_polkadot_parachain()
            .paras_info(&expected_para_id);
        match self.subxt().storage().fetch(&address, hash).await? {
            Some(v) => Ok(Some(SmartCodecMapper::map_to(&v)?)),
            None => Ok(None),
        }
    }

    async fn submit_parachain_heads(
        &self,
        relay_block: (
            <Self::Chain as Chain>::BlockNumber,
            <Self::Chain as Chain>::Hash,
        ),
        parachains: Vec<(
            bridge_s2s_traits::types::ParaId,
            <Self::Chain as Chain>::Hash,
        )>,
        parachain_heads_proof: Vec<Vec<u8>>,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash> {
        let expected_relay_block = SmartCodecMapper::map_to(&relay_block)?;
        let expected_parachains = SmartCodecMapper::map_to(&parachains)?;

        let call = crate::subxt_runtime::api::tx()
            .bridge_polkadot_parachain()
            .submit_parachain_heads(
            expected_relay_block,
            expected_parachains,
            crate::subxt_runtime::api::runtime_types::bp_polkadot_core::parachains::ParaHeadsProof(
                parachain_heads_proof,
            ),
        );
        let track = self
            .subxt()
            .tx()
            .sign_and_submit_then_watch_default(&call, self.account().signer())
            .await?;
        let events = track.wait_for_finalized_success().await.map_err(|e| {
            S2SClientError::RPC(format!(
                "send transaction failed {}: {:?}",
                <Self as ClientCommon>::CHAIN,
                e
            ))
        })?;
        Ok(events.extrinsic_hash())
    }
}
