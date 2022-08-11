use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct CollectedEnoughNewMessageRootSignaturesRunner {
    source: EcdsaSource,
}

impl CollectedEnoughNewMessageRootSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectedEnoughNewMessageRootSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}
