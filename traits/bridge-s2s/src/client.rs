use bp_messages::Weight;
use core::fmt::Debug;
use std::ops::RangeInclusive;

use client_common_traits::ClientCommon;
use codec::{Codec, Decode, Encode, EncodeLike};
use sp_runtime::generic::{Block, SignedBlock};
use sp_runtime::traits::{Extrinsic, MaybeSerializeDeserialize};
use subxt::rpc::Subscription;

use crate::error::S2SClientResult;
use crate::types::bp_runtime::Chain;

/// S2S bridge client types defined
pub trait S2SClientBase: ClientCommon {
    type Extrinsic: Send
        + Sync
        + Codec
        + EncodeLike
        + Clone
        + Eq
        + Extrinsic
        + Debug
        + MaybeSerializeDeserialize;
}

/// S2S bridge client generic trait
#[async_trait::async_trait]
pub trait S2SClientGeneric: S2SClientBase {
    /// initialization data
    type InitializationData: Encode + Decode;

    /// subscribe grandpa justifications
    async fn subscribe_grandpa_justifications(
        &self,
    ) -> S2SClientResult<Subscription<sp_core::Bytes>>;

    /// query header by hash
    async fn header(
        &self,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<<Self::Chain as Chain>::Header>>;

    /// query block by hash
    // -> SignedBlock<Block<<Self::Chain as Chain>::Header, Self::Extrinsic>>
    async fn block(
        &self,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<SignedBlock<Block<<Self::Chain as Chain>::Header, Self::Extrinsic>>>>;

    /// read proof
    async fn read_proof(
        &self,
        storage_keys: Vec<Vec<u8>>,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Vec<Vec<u8>>>;

    /// prepare initialization data
    async fn prepare_initialization_data(&self) -> S2SClientResult<Self::InitializationData>;
}

/// S2S bridge header/message api
#[async_trait::async_trait]
pub trait S2SClientRelay: S2SClientGeneric {
    /// generate outbound messages storage key
    fn gen_outbound_messages_storage_key(&self, lane: [u8; 4], message_nonce: u64) -> Vec<u8>;

    /// generate outbound lanes storage key
    fn gen_outbound_lanes_storage_key(&self, lane: [u8; 4]) -> Vec<u8>;

    /// generate inbound lanes storage key
    fn gen_inbound_lanes_storage_key(&self, lane: [u8; 4]) -> Vec<u8>;

    /// calculate dispatchh width by message nonces
    async fn calculate_dispatch_weight(
        &self,
        lane: [u8; 4],
        nonces: RangeInclusive<u64>,
    ) -> S2SClientResult<u64>;

    /// query best target finalized at source
    async fn best_target_finalized(
        &self,
        at_block: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<
        Option<(
            <Self::Chain as Chain>::BlockNumber,
            <Self::Chain as Chain>::Hash,
        )>,
    >;

    /// initialize bridge
    async fn initialize(
        &self,
        initialization_data: <Self as S2SClientGeneric>::InitializationData,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash>;

    /// submit finality proof
    async fn submit_finality_proof(
        &self,
        finality_target: <Self::Chain as Chain>::Header,
        justification: bp_header_chain::justification::GrandpaJustification<
            <Self::Chain as Chain>::Header,
        >,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash>;

    /// query outbound lane
    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<bp_messages::OutboundLaneData>;

    /// query inbound lane
    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<bp_messages::InboundLaneData<<Self::Chain as Chain>::AccountId>>;

    /// query oubound message data
    async fn outbound_messages(
        &self,
        message_key: bp_messages::MessageKey,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<bp_messages::MessageData<u128>>>;

    /// send receive messages proof extrinsics
    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: <Self::Chain as Chain>::AccountId,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<
            <Self::Chain as Chain>::Hash,
        >,
        messages_count: u32,
        dispatch_weight: Weight,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash>;

    /// receive messages delivery proof
    async fn receive_messages_delivery_proof(
        &self,
        proof: bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
            <Self::Chain as Chain>::Hash,
        >,
        relayers_state: bp_messages::UnrewardedRelayersState,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash>;
}

/// S2S with parachain bridge api for solo chain
#[cfg(feature = "bridge-parachain")]
#[async_trait::async_trait]
pub trait S2SParaBridgeClientSolochain: S2SClientRelay {
    /// beat para heads
    async fn best_para_heads(
        &self,
        para_id: crate::types::ParaId,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<crate::types::ParaInfo>>;

    /// submit parachain heads
    async fn submit_parachain_heads(
        &self,
        relay_block: (
            <Self::Chain as Chain>::BlockNumber,
            <Self::Chain as Chain>::Hash,
        ),
        parachains: Vec<(crate::types::ParaId, <Self::Chain as Chain>::Hash)>,
        parachain_heads_proof: Vec<Vec<u8>>,
    ) -> S2SClientResult<<Self::Chain as Chain>::Hash>;
}

/// S2S with parachain bridge api for relay chain
#[cfg(feature = "bridge-parachain")]
#[async_trait::async_trait]
pub trait S2SParaBridgeClientRelaychain: S2SClientGeneric {
    /// generate parachain head storage key
    fn gen_parachain_head_storage_key(&self, para_id: u32) -> Vec<u8>;

    /// query head data
    async fn para_head_data(
        &self,
        para_id: crate::types::ParaId,
        hash: Option<<Self::Chain as Chain>::Hash>,
    ) -> S2SClientResult<Option<crate::types::HeadData>>;
}
