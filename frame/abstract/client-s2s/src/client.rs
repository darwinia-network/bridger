use codec::{Decode, Encode};

/// S2S bridge client types defined
pub trait S2SClientBase {
    /// error type
    type Error;
    /// header
    type Header;
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
pub trait S2sClientRelay: S2SClientGeneric {}
