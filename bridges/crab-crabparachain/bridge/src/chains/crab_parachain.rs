pub use s2s_const::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_crab_parachain_client::CrabParachainChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for CrabParachainChain {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_runtime::create_runtime_str!("Crab Parachain"),
            impl_name: sp_runtime::create_runtime_str!("Crab Parachain"),
            authoring_version: 1,
            spec_version: 3,
            impl_version: 1,
            apis: sp_version::create_apis_vec![[]],
            transaction_version: 1,
        };

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_messages {
    use crate::feemarket::CrabParachainFeemarketApi;
    use feemarket_s2s::relay::BasicRelayStrategy;
    use frame_support::weights::Weight;
    use relay_crab_client::CrabChain;
    use relay_crab_parachain_client::CrabParachainChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    #[derive(Clone, Debug)]
    pub struct CrabParachainMessagesToCrab;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        CrabParachainMessagesToCrab,
        CrabParachainMessagesToCrabReceiveMessagesProofCallBuilder,
        relay_crab_client::runtime::Call::BridgeCrabParachainMessages,
        relay_crab_client::runtime::BridgeCrabParachainMessagesCall::receive_messages_proof
    );

    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        CrabParachainMessagesToCrab,
        CrabParachainMessagesToCrabReceiveMessagesDeliveryProofCallBuilder,
        relay_crab_parachain_client::runtime::Call::BridgeCrabMessages,
        relay_crab_parachain_client::runtime::BridgeCrabMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for CrabParachainMessagesToCrab {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = CrabParachainChain;
        type TargetChain = CrabChain;

        type SourceTransactionSignScheme = CrabParachainChain;
        type TargetTransactionSignScheme = CrabChain;

        type ReceiveMessagesProofCallBuilder =
            CrabParachainMessagesToCrabReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            CrabParachainMessagesToCrabReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = BasicRelayStrategy<CrabParachainFeemarketApi>;
    }
}

pub mod s2s_feemarket {
    use codec::Encode;
    use relay_crab_parachain_client::runtime as crab_parachain_runtime;
    use relay_crab_parachain_client::CrabParachainChain;
    use relay_substrate_client::{
        ChainBase, Client, SignParam, TransactionSignScheme, UnsignedTransaction,
    };
    use sp_core::{Bytes, Pair};

    use feemarket_s2s::error::FeemarketResult;

    pub(crate) async fn update_relay_fee(
        client: &Client<CrabParachainChain>,
        signer: <CrabParachainChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabParachainChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabParachainChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            crab_parachain_runtime::Call::CrabFeemarket(
                                crab_parachain_runtime::FeemarketCall::update_relay_fee(amount),
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
        client: &Client<CrabParachainChain>,
        signer: <CrabParachainChain as TransactionSignScheme>::AccountKeyPair,
        amount: <CrabParachainChain as ChainBase>::Balance,
    ) -> FeemarketResult<()> {
        let signer_id = (*signer.public().as_array_ref()).into();
        let genesis_hash = *client.genesis_hash();
        let (spec_version, transaction_version) = client.simple_runtime_version().await?;
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabParachainChain::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
                        genesis_hash,
                        signer: signer.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            crab_parachain_runtime::Call::CrabFeemarket(
                                crab_parachain_runtime::FeemarketCall::update_locked_collateral(
                                    amount,
                                ),
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
