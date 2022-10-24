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
        let client_pangoro_substrate = &self.source.client_darwinia_substrate;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let pangoro_evm_account = &self.source.darwinia_evm_account;

        let cacse = subquery
            .next_collecting_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-goerli",
                "[pangoro] [ecdsa] no more authorities change events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        if !client_pangoro_substrate
            .is_ecdsa_authority(Some(event.block_number), &pangoro_evm_account.address()?.0)
            .await?
        {
            tracing::warn!(
                target: "pangoro-goerli",
                "[pangoro] [ecdsa] you are not authority account. nothing to do."
            );
            return Ok(Some(event.block_number));
        }

        let address = pangoro_evm_account.address()?;
        let signature = pangoro_evm_account.sign(event.message.as_slice())?;
        let hash = client_pangoro_substrate
            .submit_authorities_change_signature(address.0, signature)
            .await?;
        tracing::info!(
            target: "pangoro-goerli",
            "[pangoro] [ecdsa] submitted new message root signature: {}",
            array_bytes::bytes2hex("0x", hash.as_ref()),
        );
        Ok(Some(event.block_number))
    }
}
