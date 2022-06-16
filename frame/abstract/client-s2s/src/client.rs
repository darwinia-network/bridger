use codec::{Decode, Encode};

/// S2S Bridge client
#[async_trait::async_trait]
pub trait S2SClient {
    /// error type
    type Error;
    /// header
    type Header;
    /// initialization data
    type InitializationData: Encode + Decode;

    /// prepare initialization data
    async fn prepare_initialization_data(&self) -> Result<Self::InitializationData, Self::Error>;
}
