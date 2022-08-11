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
    pub async fn start(&self) -> color_eyre::Result<Option<u32>> {
        let client = &self.source.client;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();

        let cacse = subquery
            .next_collecting_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] not more authorities change events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        if !client.is_ecdsa_authority(Some(event.block_number)).await? {
            tracing::warn!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] you are not authority account. nothing to do."
            );
            return Ok(Some(event.block_number));
        }

        client
            .runtime()
            .tx()
            .ecdsa_authority()
            .submit_authorities_change_signature()
            .await?;

        let acts = client
            .runtime()
            .storage()
            .ecdsa_authority()
            .authorities_change_to_sign(None)
            .await?;
        Ok(())
    }
}
