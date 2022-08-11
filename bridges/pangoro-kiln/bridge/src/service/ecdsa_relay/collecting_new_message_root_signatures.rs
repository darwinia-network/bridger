use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct CollectingNewMessageRootSignaturesRunner {
    source: EcdsaSource,
}

impl CollectingNewMessageRootSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectingNewMessageRootSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}
