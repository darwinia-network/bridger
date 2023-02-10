use bridge_e2e_traits::client::EcdsaClient;

use crate::error::RelayResult;

use super::types::EcdsaSource;

pub struct CollectingAuthoritiesChangeSignaturesRunner<T: EcdsaClient> {
    source: EcdsaSource<T>,
}

impl<T: EcdsaClient> CollectingAuthoritiesChangeSignaturesRunner<T> {
    pub fn new(source: EcdsaSource<T>) -> Self {
        Self { source }
    }
}

impl<T: EcdsaClient> CollectingAuthoritiesChangeSignaturesRunner<T> {
    pub async fn start(&self) -> RelayResult<Option<u32>> {
        let client_darwinia_substrate = &self.source.client_darwinia_substrate;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let darwinia_evm_account = &self.source.darwinia_evm_account;

        let cacse = subquery
            .next_collecting_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectingAuthorities] no more events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        let collected = subquery
            .next_collected_enough_authorities_change_signatures_event(event.block_number)
            .await?;
        if collected.is_some() {
            tracing::debug!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectingAuthorities] Collected. Skip this event at {}",
                event.block_number,
            );
            return Ok(Some(event.block_number));
        }
        if !client_darwinia_substrate
            .is_ecdsa_authority(Some(event.block_number), &darwinia_evm_account.address()?.0)
            .await?
        {
            tracing::warn!(
                target: "relay-e2e",
                "[Darwinia][ECDSA][collectingAuthorities] you are not authority account. nothing to do."
            );
            return Ok(Some(event.block_number));
        }

        let signature = darwinia_evm_account.sign(event.message.as_slice())?;
        let hash = client_darwinia_substrate
            .submit_authorities_change_signature(signature)
            .await?;
        tracing::info!(
            target: "relay-e2e",
            "[Darwinia][ECDSA][collectingAuthorities] submitted signature: {}",
            array_bytes::bytes2hex("0x", hash.as_ref()),
        );
        Ok(Some(event.block_number))
    }
}
