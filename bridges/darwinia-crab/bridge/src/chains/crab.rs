pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use relay_crab_client::CrabChain;
    use sp_version::RuntimeVersion;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for CrabChain {
        const RUNTIME_VERSION: RuntimeVersion = bp_crab::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use relay_crab_client::CrabChain;
    use relay_darwinia_client::DarwiniaChain;
    use substrate_relay_helper::finality_pipeline::SubstrateFinalitySyncPipeline;

    /// Description of Crab -> Darwinia finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct CrabFinalityToDarwinia;

    substrate_relay_helper::generate_mocked_submit_finality_proof_call_builder!(
        CrabFinalityToDarwinia,
        CrabFinalityToDarwiniaCallBuilder,
        relay_darwinia_client::runtime::Call::BridgeCrabGrandpa,
        relay_darwinia_client::runtime::BridgeCrabGrandpaCall::submit_finality_proof
    );

    impl SubstrateFinalitySyncPipeline for CrabFinalityToDarwinia {
        type SourceChain = CrabChain;
        type TargetChain = DarwiniaChain;

        type SubmitFinalityProofCallBuilder = CrabFinalityToDarwiniaCallBuilder;
        type TransactionSignScheme = DarwiniaChain;
    }

    // === end
}

mod s2s_messages {
    use frame_support::weights::Weight;
    use relay_crab_client::CrabChain;
    use relay_darwinia_client::DarwiniaChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    use feemarket_s2s_transition::relay::BasicRelayStrategy;

    use crate::feemarket::CrabFeemarketApi;

    #[derive(Clone, Debug)]
    pub struct CrabMessagesToDarwinia;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        CrabMessagesToDarwinia,
        CrabMessagesToDarwiniaReceiveMessagesProofCallBuilder,
        relay_darwinia_client::runtime::Call::BridgeCrabMessages,
        relay_darwinia_client::runtime::BridgeCrabMessagesCall::receive_messages_proof
    );
    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        CrabMessagesToDarwinia,
        CrabMessagesToDarwiniaReceiveMessagesDeliveryProofCallBuilder,
        relay_crab_client::runtime::Call::BridgeDarwiniaMessages,
        relay_crab_client::runtime::BridgeDarwiniaMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for CrabMessagesToDarwinia {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = CrabChain;
        type TargetChain = DarwiniaChain;

        type SourceTransactionSignScheme = CrabChain;
        type TargetTransactionSignScheme = DarwiniaChain;

        type ReceiveMessagesProofCallBuilder =
            CrabMessagesToDarwiniaReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            CrabMessagesToDarwiniaReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = BasicRelayStrategy<CrabFeemarketApi>;
    }
}

pub mod s2s_feemarket {
    use codec::Encode;
    use relay_crab_client::runtime as crab_runtime;
    use relay_crab_client::CrabChain;
    use relay_substrate_client::{
        ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
    };
    use sp_core::{Bytes, Pair};

    use feemarket_s2s_transition::error::FeemarketResult;

    pub(crate) async fn update_relay_fee(
        client: &Client<CrabChain>,
        signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            crab_runtime::Call::Feemarket(
                                crab_runtime::FeemarketCall::update_relay_fee(amount),
                            ),
                            transaction_nonce,
                        ),
                    })
                    .encode(),
                )
            })
            .await?;
        Ok(())
    }

    pub(crate) async fn update_locked_collateral(
        client: &Client<CrabChain>,
        signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            crab_runtime::Call::Feemarket(
                                crab_runtime::FeemarketCall::update_locked_collateral(amount),
                            ),
                            transaction_nonce,
                        ),
                    })
                    .encode(),
                )
            })
            .await?;
        Ok(())
    }
}
