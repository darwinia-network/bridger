use core::fmt::Debug;
use std::ops::RangeInclusive;

use codec::{Codec, Decode, Encode, EncodeLike};
use jsonrpsee_core::client::Subscription;
use sp_runtime::generic::{Block, SignedBlock};
use sp_runtime::traits::{Extrinsic, MaybeSerializeDeserialize};

use crate::config::Config;
use crate::error::S2SClientResult;

/// S2S bridge client types defined
pub trait S2SClientBase: Send + Sync + 'static {
    const CHAIN: &'static str;
    type Config: Config;
    type Extrinsic: Codec + EncodeLike + Clone + Eq + Extrinsic + Debug + MaybeSerializeDeserialize;
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

    /// prepare initialization data
    async fn prepare_initialization_data(&self) -> S2SClientResult<Self::InitializationData>;

    /// initialize bridge
    async fn initialize(
        &self,
        initialization_data: Self::InitializationData,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;
}

/// S2S bridge header/message api
#[async_trait::async_trait]
pub trait S2SClientRelay: S2SClientGeneric {
    /// generate outbound messages storage key
    fn gen_outbound_messages_storage_key(
        &self,
        lane: [u8; 4],
        message_nonce: u64,
    ) -> sp_core::storage::StorageKey;

    /// generate outbound lanes storage key
    fn gen_outbound_lanes_storage_key(&self, lane: [u8; 4]) -> sp_core::storage::StorageKey;

    /// generate inbound lanes storage key
    fn gen_inbound_lanes_storage_key(&self, lane: [u8; 4]) -> sp_core::storage::StorageKey;

    /// calculate dispatchh width by message nonces
    async fn calculate_dispatch_weight(
        &self,
        lane: [u8; 4],
        nonces: RangeInclusive<u64>,
    ) -> S2SClientResult<u64>;

    /// query header by hash
    async fn header(
        &self,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Option<<Self::Config as Config>::Header>>;

    /// query block by hash
    async fn block(
        &self,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<
        Option<SignedBlock<Block<<Self::Config as Config>::Header, Self::Extrinsic>>>,
    >;

    /// query best target finalized at source
    async fn best_target_finalized(
        &self,
        at_block: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;

    /// submit finality proof
    async fn submit_finality_proof(
        &self,
        finality_target: <Self::Config as Config>::Header,
        justification: bp_header_chain::justification::GrandpaJustification<
            <Self::Config as Config>::Header,
        >,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;

    /// query outbound lane
    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<bp_messages::OutboundLaneData>;

    /// query inbound lane
    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<bp_messages::InboundLaneData<sp_core::crypto::AccountId32>>;

    /// query oubound message data
    async fn outbound_messages(
        &self,
        message_key: bp_messages::MessageKey,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Option<bp_messages::MessageData<u128>>>;

    /// read proof
    async fn read_proof(
        &self,
        storage_keys: Vec<sp_core::storage::StorageKey>,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Vec<Vec<u8>>>;

    /// send receive messages proof extrinsics
    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: sp_core::crypto::AccountId32,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<
            <Self::Config as Config>::Hash,
        >,
        messages_count: u32,
        dispatch_weight: u64,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;

    /// receive messages delivery proof
    async fn receive_messages_delivery_proof(
        &self,
        proof: bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
            <Self::Config as Config>::Hash,
        >,
        relayers_state: bp_messages::UnrewardedRelayersState,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;
}

/// S2S with parachain bridge api for solo chain
#[cfg(feature = "bridge-parachain")]
#[async_trait::async_trait]
pub trait S2SParaBridgeClientSolochain: S2SClientRelay {
    /// beat para heads
    async fn best_para_heads(
        &self,
        para_id: crate::types::ParaId,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Option<crate::types::BestParaHead>>;

    /// submit parachain heads
    async fn submit_parachain_heads(
        &self,
        relay_block_hash: <Self::Config as Config>::Hash,
        parachains: Vec<crate::types::ParaId>,
        parachain_heads_proof: Vec<Vec<u8>>,
    ) -> S2SClientResult<<Self::Config as Config>::Hash>;
}

/// S2S with parachain bridge api for relay chain
#[cfg(feature = "bridge-parachain")]
#[async_trait::async_trait]
pub trait S2SParaBridgeClientRelaychain: S2SClientGeneric {
    /// generate parachain head storage key
    fn gen_parachain_head_storage_key(&self, para_id: u32) -> sp_core::storage::StorageKey;

    /// query head data
    async fn para_head_data(
        &self,
        para_id: crate::types::ParaId,
        hash: Option<<Self::Config as Config>::Hash>,
    ) -> S2SClientResult<Option<crate::types::HeadData>>;
}
