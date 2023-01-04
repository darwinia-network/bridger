use std::fmt::Debug;

use codec::Encode;
use sp_runtime::traits::SignedExtension;

#[derive(Debug)]
pub struct DarwiniaLikeExtrinsicParams {
    signed_extensions: bp_darwinia_core::SignedExtensions<()>,
}

impl subxt::tx::ExtrinsicParams<bp_darwinia_core::Nonce, bp_darwinia_core::Hash>
    for DarwiniaLikeExtrinsicParams
{
    type OtherParams = DarwiniaLikeExtrinsicParamsBuilder;

    fn new(
        spec_version: u32,
        tx_version: u32,
        nonce: bp_darwinia_core::Nonce,
        genesis_hash: bp_darwinia_core::Hash,
        other_params: Self::OtherParams,
    ) -> Self {
        let ext = bp_darwinia_core::SignedExtensions::new(
            spec_version,
            tx_version,
            other_params.era,
            genesis_hash,
            nonce,
            other_params.tip,
        );
        Self {
            signed_extensions: ext,
        }
    }

    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.signed_extensions.encode_to(v)
    }

    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        if let Ok(additional) = self.signed_extensions.additional_signed() {
            additional.encode_to(v);
        }
    }
}

#[derive(Debug)]
pub struct DarwiniaLikeExtrinsicParamsBuilder {
    era: bp_runtime::TransactionEraOf<bp_darwinia_core::DarwiniaLike>,
    _mortality_checkpoint: Option<bp_darwinia_core::Hash>,
    tip: bp_darwinia_core::Balance,
}

impl Default for DarwiniaLikeExtrinsicParamsBuilder {
    fn default() -> Self {
        Self {
            era: bp_runtime::TransactionEraOf::<bp_darwinia_core::DarwiniaLike>::Immortal,
            _mortality_checkpoint: None,
            tip: Default::default(),
        }
    }
}
