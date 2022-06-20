use abstract_client_s2s::client::S2SClientRelay;
use sp_runtime::AccountId32;
use subxt::rpc::ChainBlock;
use subxt::sp_core::storage::StorageKey;

use crate::client::PangoroClient;
use crate::config::PangoroSubxtConfig;
use crate::error::ClientResult;
use crate::types::runtime_types::bp_messages;
use crate::types::runtime_types::bridge_runtime_common;

type BundleJustification =
    crate::types::runtime_types::bp_header_chain::justification::GrandpaJustification<
        crate::fastapi::s2s::generic::BundleHeader,
    >;

#[async_trait::async_trait]
impl S2SClientRelay for PangoroClient {
    type Justification = BundleJustification;
    type ChainBlock = ChainBlock<PangoroSubxtConfig>;
    type OutboundLaneData = bp_messages::OutboundLaneData;
    type InboundLaneData = bp_messages::InboundLaneData<subxt::sp_core::crypto::AccountId32>;
    type MessageData = bp_messages::MessageData<u128>;
    type MessageKey = bp_messages::MessageKey;
    type StorageKey = StorageKey;
    type BridgedChainMessagesProof =
        bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<Self::Hash>;
    type BridgedChainMessagesDeliveryProof =
        bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<Self::Hash>;
    type UnrewardedRelayersState = bp_messages::UnrewardedRelayersState;

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
    ) -> ClientResult<subxt::sp_core::H256> {
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
        justification: Self::Justification,
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
    ) -> ClientResult<Self::OutboundLaneData> {
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
    ) -> ClientResult<Self::InboundLaneData> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .inbound_lanes(lane, hash)
            .await?)
    }

    async fn outbound_messages(
        &self,
        message_key: Self::MessageKey,
        hash: Option<Self::Hash>,
    ) -> ClientResult<Option<Self::MessageData>> {
        Ok(self
            .runtime()
            .storage()
            .bridge_pangolin_messages()
            .outbound_messages(message_key, hash)
            .await?)
    }

    async fn read_proof(
        &self,
        storage_keys: Vec<Self::StorageKey>,
        hash: Option<Self::Hash>,
    ) -> ClientResult<Vec<Vec<u8>>> {
        let read_proof = self.subxt().rpc().read_proof(storage_keys, hash).await?;
        let proof: Vec<Vec<u8>> = read_proof.proof.into_iter().map(|item| item.0).collect();
        Ok(proof)
    }

    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: AccountId32,
        proof: Self::BridgedChainMessagesProof,
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
        proof: Self::BridgedChainMessagesDeliveryProof,
        relayers_state: Self::UnrewardedRelayersState,
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
