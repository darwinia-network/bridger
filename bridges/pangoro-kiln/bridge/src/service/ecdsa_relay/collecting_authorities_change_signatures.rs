use web3::signing::SecretKeyRef;

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
        let client_pangoro_substrate = &self.source.client_pangoro_substrate;
        let client_pangoro_web3 = &self.source.client_pangoro_web3;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let eth_account = &self.source.pangoro_evm_account;

        let cacse = subquery
            .next_collecting_authorities_change_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] no more authorities change events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        if !client_pangoro_substrate
            .is_ecdsa_authority(Some(event.block_number), &eth_account.address()?.0)
            .await?
        {
            tracing::warn!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] you are not authority account. nothing to do."
            );
            return Ok(Some(event.block_number));
        }

        let signature = client_pangoro_web3
            .accounts()
            .sign(
                event.message.as_slice(),
                SecretKeyRef::new(&eth_account.secret_key()?),
            )
            .signature;

        let address = eth_account.address()?;
        let _hash = client_pangoro_substrate
            .submit_authorities_change_signature(address.0, signature.0)
            .await?;

        Ok(Some(event.block_number))
    }
}
