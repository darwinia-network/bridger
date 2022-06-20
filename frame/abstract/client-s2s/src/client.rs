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
    /// Justification
    type Justification;
    /// Chain block
    type ChainBlock;
    /// outbound lane data
    type OutboundLaneData;
    /// inbound lane data
    type InboundLaneData;
    /// message data
    type MessageData;
    /// message key
    type MessageKey;
    /// storage key
    type StorageKey;
    /// bridged chain messages proof
    type BridgedChainMessagesProof;
    /// bridged chain message delivery proof
    type BridgedChainMessagesDeliveryProof;
    /// unrewarded relayers state
    type UnrewardedRelayersState;

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
    ) -> Result<sp_core::H256, Self::Error>;

    /// submit finality proof
    async fn submit_finality_proof(
        &self,
        finality_target: Self::Header,
        justification: Self::Justification,
    ) -> Result<Self::Hash, Self::Error>;

    /// query outbound lane
    async fn outbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> Result<Self::OutboundLaneData, Self::Error>;

    /// query inbound lane
    async fn inbound_lanes(
        &self,
        lane: [u8; 4],
        hash: Option<Self::Hash>,
    ) -> Result<Self::InboundLaneData, Self::Error>;

    /// query oubound message data
    async fn outbound_messages(
        &self,
        message_key: Self::MessageKey,
        hash: Option<Self::Hash>,
    ) -> Result<Self::MessageData, Self::Error>;

    /// read proof
    async fn read_proof(
        &self,
        storage_keys: Vec<Self::StorageKey>,
        hash: Option<Self::Hash>,
    ) -> Result<Self::BridgedChainMessagesProof, Self::Error>;

    /// send receive messages proof extrinsics
    async fn receive_messages_proof(
        &self,
        relayer_id_at_bridged_chain: sp_core::crypto::AccountId32,
        proof: Self::BridgedChainMessagesProof,
        messages_count: u32,
        dispatch_weight: u64,
    ) -> Result<Self::Hash, Self::Error>;

    /// receive messages delivery proof
    async fn receive_messages_delivery_proof(
        &self,
        proof: Self::BridgedChainMessagesDeliveryProof,
        relayers_state: Self::UnrewardedRelayersState,
    ) -> Result<Self::Hash, Self::Error>;
}
