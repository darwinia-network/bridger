use std::ops::RangeInclusive;

use codec::{Decode, Encode};

/// S2S bridge client types defined
pub trait S2SClientBase {
    /// error type
    type Error;
    /// header
    type Header;
    /// hash
    type Hash;
}

/// S2S bridge client generic trait
#[async_trait::async_trait]
pub trait S2SClientGeneric: S2SClientBase {
    /// initialization data
    type InitializationData: Encode + Decode;

    /// prepare initialization data
    async fn prepare_initialization_data(&self) -> Result<Self::InitializationData, Self::Error>;
}

/// S2S bridge header/message api
#[async_trait::async_trait]
pub trait S2SClientRelay: S2SClientGeneric {
    /// Chain block
    type ChainBlock;

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
    async fn calculate_dispatch_width(
        &self,
        nonces: RangeInclusive<u64>,
    ) -> Result<u64, Self::Error>;

    /// query header by hash
    async fn header(&self, hash: Option<Self::Hash>) -> Result<Option<Self::Header>, Self::Error>;

    /// query block by hash
    async fn block(
        &self,
        hash: Option<Self::Hash>,
    ) -> Result<Option<Self::ChainBlock>, Self::Error>;

    /// query best target finalized at source
    async fn best_target_finalized(
        &self,
        at_block: Option<Self::Hash>,
    ) -> Result<Self::Hash, Self::Error>;

    /// submit finality proof
    async fn submit_finality_proof(
        &self,
        finality_target: Self::Header,
        justification: bp_header_chain::justification::GrandpaJustification<Self::Header>,
    ) -> Result<Self::Hash, Self::Error>;

    /// query outbound lane
    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> Result<bp_messages::OutboundLaneData, Self::Error>;

    /// query inbound lane
    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> Result<bp_messages::InboundLaneData<sp_core::crypto::AccountId32>, Self::Error>;

    /// query oubound message data
    async fn outbound_messages(
        &self,
        message_key: bp_messages::MessageKey,
        hash: Option<Self::Hash>,
    ) -> Result<Option<bp_messages::MessageData<u128>>, Self::Error>;

    /// read proof
    async fn read_proof(
        &self,
        storage_keys: Vec<sp_core::storage::StorageKey>,
        hash: Option<Self::Hash>,
    ) -> Result<Vec<Vec<u8>>, Self::Error>;

    /// send receive messages proof extrinsics
    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: sp_core::crypto::AccountId32,
        proof: bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<Self::Hash>,
        messages_count: u32,
        dispatch_weight: u64,
    ) -> Result<Self::Hash, Self::Error>;

    /// receive messages delivery proof
    async fn receive_messages_delivery_proof(
        &self,
        proof: bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
            Self::Hash,
        >,
        relayers_state: bp_messages::UnrewardedRelayersState,
    ) -> Result<Self::Hash, Self::Error>;
}
