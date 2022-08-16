use crate::service::ecdsa_relay::types::EcdsaSource;
use web3::signing::SecretKeyRef;

pub struct CollectingNewMessageRootSignaturesRunner {
    source: EcdsaSource,
}

impl CollectingNewMessageRootSignaturesRunner {
    pub fn new(source: EcdsaSource) -> Self {
        Self { source }
    }
}

impl CollectingNewMessageRootSignaturesRunner {
    pub async fn start(&self) -> color_eyre::Result<Option<u32>> {
        let client_pangoro_substrate = &self.source.client_pangoro_substrate;
        let client_pangoro_web3 = &self.source.client_pangoro_web3;
        let subquery = &self.source.subquery;
        let from_block = self.source.block.unwrap_or_default();
        let pangoro_evm_account = &self.source.pangoro_evm_account;

        let cacse = subquery
            .next_collecting_new_message_root_signatures_event(from_block)
            .await?;
        if cacse.is_none() {
            tracing::debug!(
                target: "pangoro-kiln",
                "[pangoro] [ecdsa] no more new message root signatures events after {}",
                from_block,
            );
            return Ok(None);
        }
        let event = cacse.expect("Unreachable");
        tracing::info!(
            target: "pangoro-kiln",
            "[pangoro] [ecdsa] found new message root signature event from block {}",
            event.block_number,
        );
        if !client_pangoro_substrate
            .is_ecdsa_authority(Some(event.block_number), &pangoro_evm_account.address()?.0)
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
                SecretKeyRef::new(&pangoro_evm_account.secret_key()?),
            )
            .signature;

        let address = pangoro_evm_account.address()?;
        let _hash = client_pangoro_substrate
            .submit_new_message_root_signature(address.0, signature.0)
            .await?;

        Ok(Some(event.block_number))
    }
}
