use std::ops::RangeInclusive;

use abstract_client_s2s::{
    client::S2SClientRelay,
    types::{bp_header_chain, bp_messages, bridge_runtime_common},
};
use sp_runtime::AccountId32;
use subxt::rpc::ChainBlock;
use subxt::sp_core::storage::StorageKey;
use subxt::storage::StorageKeyPrefix;
use subxt::StorageEntry;

use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::ClientError;
use crate::error::ClientResult;
use crate::subxt_runtime::api::bridge_pangolin_messages::storage::{
    InboundLanes, OutboundLanes, OutboundMessages,
};

type BundleMessageKey = crate::types::runtime_types::bp_messages::MessageKey;
type BundleJustification =
    crate::types::runtime_types::bp_header_chain::justification::GrandpaJustification<
        crate::fastapi::s2s::generic::BundleHeader,
    >;

/// Message payload for This -> Bridged chain messages.
type FromThisChainMessagePayload = crate::types::runtime_types::bp_message_dispatch::MessagePayload<
    sp_core::crypto::AccountId32,
    crate::types::runtime_types::sp_runtime::MultiSigner,
    crate::types::runtime_types::sp_runtime::MultiSignature,
    Vec<u8>,
>;

#[async_trait::async_trait]
impl S2SClientRelay for PangoroClient {
    type ChainBlock = ChainBlock<PangoroSubxtConfig>;

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
    ) -> Result<u64, Self::Error> {
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
                        "Can not read message data by nonce {} in pangolin",
                        message_nonce
                    ))
                })?;
            let decoded_payload: FromThisChainMessagePayload =
                codec::Decode::decode(&mut &message_data.payload[..])?;
            total_weight += decoded_payload.weight;
        }
        Ok(total_weight)
    }

    async fn header(&self, hash: Option<Self::Hash>) -> ClientResult<Option<Self::Header>> {
        match self.subxt().rpc().header(hash).await? {
            Some(v) => {
                let v = codec::Encode::encode(&v);
                Ok(Some(codec::Decode::decode(&mut v.as_slice())?))
            }
            None => Ok(None),
        }
    }

    async fn block(&self, hash: Option<Self::Hash>) -> ClientResult<Option<Self::ChainBlock>> {
        Ok(self.subxt().rpc().block(hash).await?)
    }

    async fn best_target_finalized(
        &self,
        at_block: Option<Self::Hash>,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_grandpa()
            .best_finalized(at_block)
            .await?)
    }

    async fn submit_finality_proof(
        &self,
        finality_target: Self::Header,
        justification: bp_header_chain::justification::GrandpaJustification<Self::Header>,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .tx()
            .bridge_pangolin_grandpa()
            .submit_finality_proof(finality_target, justification)
            .sign_and_submit(self.account().signer())
            .await?)
    }

    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> ClientResult<bp_messages::OutboundLaneData> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_lanes(lane, hash)
            .await?)
    }

    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> ClientResult<bp_messages::InboundLaneData<sp_core::crypto::AccountId32>> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .inbound_lanes(lane, hash)
            .await?)
    }

    async fn outbound_messages(
        &self,
        message_key: bp_messages::MessageKey,
        hash: Option<Self::Hash>,
    ) -> ClientResult<Option<bp_messages::MessageData<u128>>> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_messages(message_key, hash)
            .await?)
    }

    async fn read_proof(
        &self,
        storage_keys: Vec<sp_core::storage::StorageKey>,
        hash: Option<Self::Hash>,
    ) -> ClientResult<Vec<Vec<u8>>> {
        let read_proof = self.subxt().rpc().read_proof(storage_keys, hash).await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        Ok(proof)
    }

    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: AccountId32,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<Self::Hash>,
        messages_count: u32,
        dispatch_weight: u64,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .tx()
            .bridge_pangolin_messages()
            .receive_messages_proof(
                relayer_id_at_bridged_chain,
                proof,
                messages_count,
                dispatch_weight,
            )
            .sign_and_submit(self.account().signer())
            .await?)
    }

    async fn receive_messages_delivery_proof(
        &self,
        proof: bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
            Self::Hash,
        >,
        relayers_state: bp_messages::UnrewardedRelayersState,
    ) -> ClientResult<Self::Hash> {
        Ok(self
            .runtime()
            .tx()
            .bridge_pangolin_messages()
            .receive_messages_delivery_proof(proof, relayers_state)
            .sign_and_submit(self.account().signer())
            .await?)
    }
}
