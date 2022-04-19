pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use relay_pangoro_client::PangoroChain;
    use sp_version::RuntimeVersion;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangoroChain {
        const RUNTIME_VERSION: RuntimeVersion = bp_pangoro::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use relay_pangolin_client::PangolinChain;
    use relay_pangoro_client::PangoroChain;
    use substrate_relay_helper::finality_pipeline::SubstrateFinalitySyncPipeline;

    /// Description of Pangoro -> Pangolin finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct PangoroFinalityToPangolin;

    substrate_relay_helper::generate_mocked_submit_finality_proof_call_builder!(
        PangoroFinalityToPangolin,
        PangoroFinalityToPangolinCallBuilder,
        relay_pangolin_client::runtime::Call::BridgePangoroGrandpa,
        relay_pangolin_client::runtime::BridgePangoroGrandpaCall::submit_finality_proof
    );

    impl SubstrateFinalitySyncPipeline for PangoroFinalityToPangolin {
        type SourceChain = PangoroChain;
        type TargetChain = PangolinChain;

        type SubmitFinalityProofCallBuilder = PangoroFinalityToPangolinCallBuilder;
        type TransactionSignScheme = PangolinChain;
    }

    // === end
}

mod s2s_messages {
    use frame_support::weights::Weight;
    use relay_pangolin_client::PangolinChain;
    use relay_pangoro_client::PangoroChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    use feemarket_s2s::relay::BasicRelayStrategy;

    use crate::feemarket::PangoroFeemarketApi;

    #[derive(Clone, Debug)]
    pub struct PangoroMessagesToPangolin;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        PangoroMessagesToPangolin,
        PangoroMessagesToPangolinReceiveMessagesProofCallBuilder,
        relay_pangolin_client::runtime::Call::BridgePangoroMessages,
        relay_pangolin_client::runtime::BridgePangoroMessagesCall::receive_messages_proof
    );
    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        PangoroMessagesToPangolin,
        PangoroMessagesToPangolinReceiveMessagesDeliveryProofCallBuilder,
        relay_pangoro_client::runtime::Call::BridgePangolinMessages,
        relay_pangoro_client::runtime::BridgePangolinMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for PangoroMessagesToPangolin {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangoroChain;
        type TargetChain = PangolinChain;

        type SourceTransactionSignScheme = PangoroChain;
        type TargetTransactionSignScheme = PangolinChain;

        type ReceiveMessagesProofCallBuilder =
            PangoroMessagesToPangolinReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            PangoroMessagesToPangolinReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = BasicRelayStrategy<PangoroFeemarketApi>;
    }
}

pub mod s2s_feemarket {
    use codec::Encode;
    use relay_pangoro_client::runtime as pangoro_runtime;
    use relay_pangoro_client::PangoroChain;
    use relay_substrate_client::{
        ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
    };
    use sp_core::{Bytes, Pair};

    use feemarket_s2s::error::FeemarketResult;

    pub(crate) async fn update_relay_fee(
        client: &Client<PangoroChain>,
        signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangoroChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangoroChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            pangoro_runtime::Call::Feemarket(
                                pangoro_runtime::FeemarketCall::update_relay_fee(amount),
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
        client: &Client<PangoroChain>,
        signer: <PangoroChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangoroChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangoroChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            pangoro_runtime::Call::Feemarket(
                                pangoro_runtime::FeemarketCall::update_locked_collateral(amount),
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
