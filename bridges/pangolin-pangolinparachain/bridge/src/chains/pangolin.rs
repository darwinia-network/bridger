pub use s2s_const::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_pangolin_client::PangolinChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinChain {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_runtime::create_runtime_str!("Pangolin"),
            impl_name: sp_runtime::create_runtime_str!("Pangolin"),
            authoring_version: 0,
            spec_version: 28_100,
            impl_version: 0,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 0,
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_messages {
    use crate::feemarket::PangolinFeemarketApi;
    use feemarket_s2s::relay::BasicRelayStrategy;
    use frame_support::weights::Weight;
    use relay_pangolin_client::PangolinChain;
    use relay_pangolin_parachain_client::PangolinParachainChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    #[derive(Clone, Debug)]
    pub struct PangolinMessagesToPangolinParachain;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        PangolinMessagesToPangolinParachain,
        PangolinMessagesToPangolinParachainReceiveMessagesProofCallBuilder,
        relay_pangolin_parachain_client::runtime::Call::BridgePangolinMessages,
        relay_pangolin_parachain_client::runtime::BridgePangolinMessagesCall::receive_messages_proof
    );

    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        PangolinMessagesToPangolinParachain,
        PangolinMessagesToPangolinParachainReceiveMessagesDeliveryProofCallBuilder,
        relay_pangolin_client::runtime::Call::BridgePangolinParachainMessages,
        relay_pangolin_client::runtime::BridgePangolinParachainMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for PangolinMessagesToPangolinParachain {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangolinChain;
        type TargetChain = PangolinParachainChain;

        type SourceTransactionSignScheme = PangolinChain;
        type TargetTransactionSignScheme = PangolinParachainChain;

        type ReceiveMessagesProofCallBuilder =
            PangolinMessagesToPangolinParachainReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            PangolinMessagesToPangolinParachainReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = BasicRelayStrategy<PangolinFeemarketApi>;
    }
}

pub mod s2s_feemarket {
    use codec::Encode;
    use relay_pangolin_client::runtime as pangolin_runtime;
    use relay_pangolin_client::PangolinChain;
    use relay_substrate_client::{
        ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
    };
    use sp_core::{Bytes, Pair};

    use feemarket_s2s::error::FeemarketResult;

    pub(crate) async fn update_relay_fee(
        client: &Client<PangolinChain>,
        signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangolinChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangolinChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            pangolin_runtime::Call::PangolinParachainFeemarket(
                                pangolin_runtime::FeemarketCall::update_relay_fee(amount),
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
        client: &Client<PangolinChain>,
        signer: <PangolinChain as TransactionSignScheme>::AccountKeyPair,
        amount: <PangolinChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    PangolinChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            pangolin_runtime::Call::PangolinParachainFeemarket(
                                pangolin_runtime::FeemarketCall::update_locked_collateral(amount),
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
