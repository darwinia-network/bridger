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
    pub async fn start(&self) -> color_eyre::Result<Option<u32>> {
        let client_posa = &self.source.client_posa;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();

        let cacse = subquery
            .next_collected_enough_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] not more new message root signatures events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");

        // client_posa

        Ok(None)
    }
}
