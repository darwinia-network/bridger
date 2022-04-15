pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use relay_darwinia_client::DarwiniaChain;
    use sp_version::RuntimeVersion;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for DarwiniaChain {
        const RUNTIME_VERSION: RuntimeVersion = bp_darwinia::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use relay_crab_client::CrabChain;
    use relay_darwinia_client::DarwiniaChain;
    use substrate_relay_helper::finality_pipeline::SubstrateFinalitySyncPipeline;

    /// Description of Darwinia -> Crab finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct DarwiniaFinalityToCrab;

    substrate_relay_helper::generate_mocked_submit_finality_proof_call_builder!(
        DarwiniaFinalityToCrab,
        DarwiniaFinalityToCrabCallBuilder,
        relay_crab_client::runtime::Call::BridgeDarwiniaGrandpa,
        relay_crab_client::runtime::BridgeDarwiniaGrandpaCall::submit_finality_proof
    );

    impl SubstrateFinalitySyncPipeline for DarwiniaFinalityToCrab {
        type SourceChain = DarwiniaChain;
        type TargetChain = CrabChain;

        type SubmitFinalityProofCallBuilder = DarwiniaFinalityToCrabCallBuilder;
        type TransactionSignScheme = CrabChain;
    }

    // === end
}

mod s2s_messages {
    use frame_support::weights::Weight;
    use relay_crab_client::CrabChain;
    use relay_darwinia_client::DarwiniaChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    use feemarket_s2s::relay::BasicRelayStrategy;

    use crate::feemarket::DarwiniaFeemarketApi;

    #[derive(Clone, Debug)]
    pub struct DarwiniaMessagesToCrab;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        DarwiniaMessagesToCrab,
        DarwiniaMessagesToCrabReceiveMessagesProofCallBuilder,
        relay_crab_client::runtime::Call::BridgeDarwiniaMessages,
        relay_crab_client::runtime::BridgeDarwiniaMessagesCall::receive_messages_proof
    );
    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        DarwiniaMessagesToCrab,
        DarwiniaMessagesToCrabReceiveMessagesDeliveryProofCallBuilder,
        relay_darwinia_client::runtime::Call::BridgeCrabMessages,
        relay_darwinia_client::runtime::BridgeCrabMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for DarwiniaMessagesToCrab {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = DarwiniaChain;
        type TargetChain = CrabChain;

        type SourceTransactionSignScheme = DarwiniaChain;
        type TargetTransactionSignScheme = CrabChain;

        type ReceiveMessagesProofCallBuilder =
            DarwiniaMessagesToCrabReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            DarwiniaMessagesToCrabReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = BasicRelayStrategy<DarwiniaFeemarketApi>;
    }
}

pub mod s2s_feemarket {
    use codec::Encode;
    use relay_darwinia_client::runtime as darwinia_runtime;
    use relay_darwinia_client::DarwiniaChain;
    use relay_substrate_client::{
        ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
    };
    use sp_core::{Bytes, Pair};

    use feemarket_s2s::error::FeemarketResult;

    pub(crate) async fn update_relay_fee(
        client: &Client<DarwiniaChain>,
        signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
        amount: <DarwiniaChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    DarwiniaChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            darwinia_runtime::Call::Feemarket(
                                darwinia_runtime::FeemarketCall::update_relay_fee(amount),
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
        client: &Client<DarwiniaChain>,
        signer: <DarwiniaChain as TransactionSignScheme>::AccountKeyPair,
        amount: <DarwiniaChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    DarwiniaChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            darwinia_runtime::Call::Feemarket(
                                darwinia_runtime::FeemarketCall::update_locked_collateral(amount),
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
