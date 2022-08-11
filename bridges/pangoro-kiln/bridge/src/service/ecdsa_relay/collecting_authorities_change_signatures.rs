use crate::service::ecdsa_relay::types::EcdsaSource;

pub struct CollectingAuthoritiesChangeSignaturesRunner {
    source: EcdsaSource,
}

impl CollectingAuthoritiesChangeSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectingAuthoritiesChangeSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}
