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
        let client_darwinia_substrate = &self.source.client_darwinia_substrate;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let darwinia_evm_account = &self.source.darwinia_evm_account;

        let cacse = subquery
            .next_collecting_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "darwinia-goerli",
                "[darwinia] [ecdsa] no more authorities change events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        if !client_darwinia_substrate
            .is_ecdsa_authority(Some(event.block_number), &darwinia_evm_account.address()?.0)
            .await?
        {
            tracing::warn!(
                target: "darwinia-goerli",
                "[darwinia] [ecdsa] you are not authority account. nothing to do."
            );
            return Ok(Some(event.block_number));
        }

        let address = darwinia_evm_account.address()?;
        let signature = darwinia_evm_account.sign(event.message.as_slice())?;
        let hash = client_darwinia_substrate
            .submit_authorities_change_signature(address.0, signature)
            .await?;
        tracing::info!(
            target: "darwinia-goerli",
            "[darwinia] [ecdsa] submitted new message root signature: {}",
            array_bytes::bytes2hex("0x", &hash.0),
        );
        Ok(Some(event.block_number))
    }
}
