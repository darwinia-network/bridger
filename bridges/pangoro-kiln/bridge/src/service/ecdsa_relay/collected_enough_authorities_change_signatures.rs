use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct CollectedEnoughAuthoritiesChangeSignaturesRunner {
    source: EcdsaSource,
}

impl CollectedEnoughAuthoritiesChangeSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectedEnoughAuthoritiesChangeSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}
