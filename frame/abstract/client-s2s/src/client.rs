use codec::{Decode, Encode};

/// S2S bridge client types defined
pub trait S2SClientBase {
    /// error type
    type Error;
    /// header
    type Header;
    /// hash
    type Hash;
    /// initialization data
    type InitializationData: Encode + Decode;
}

/// S2S bridge client generic trait
#[async_trait::async_trait]
pub trait S2SClientGeneric: S2SClientBase {
    /// prepare initialization data
    async fn prepare_initialization_data(&self) -> Result<Self::InitializationData, Self::Error>;
}

/// S2S bridge header/message api
#[async_trait::async_trait]
pub trait S2SClientRelay: S2SClientGeneric {
    /// Chain block
    type ChainBlock;

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
}
