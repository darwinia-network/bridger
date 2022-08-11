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
        let client = &self.source.client;
        let subquery = &self.source.subquery;

        subquery.next_collecting_authorities_change_signatures_event()

        let acts = client
            .runtime()
            .storage()
            .ecdsa_authority()
            .authorities_change_to_sign(None)
            .await?;
        Ok(())
    }
}
