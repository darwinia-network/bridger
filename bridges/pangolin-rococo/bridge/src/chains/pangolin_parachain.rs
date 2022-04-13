pub use s2s_const::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use relay_pangolin_parachain_client::PangolinParachainChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinParachainChain {
        const RUNTIME_VERSION: RuntimeVersion = RuntimeVersion {
            spec_name: sp_runtime::create_runtime_str!("Pangolin Parachain"),
            impl_name: sp_runtime::create_runtime_str!("Pangolin Parachain"),
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
    use frame_support::weights::Weight;
    use relay_pangolin_client::PangolinChain;
    use relay_pangolin_parachain_client::PangolinParachainChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    #[derive(Clone, Debug)]
    pub struct PangolinParachainMessagesToPangolin;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        PangolinParachainMessagesToPangolin,
        PangolinParachainMessagesToPangolinReceiveMessagesProofCallBuilder,
        relay_pangolin_client::runtime::Call::BridgePangolinParachainMessages,
        relay_pangolin_client::runtime::BridgePangolinParachainMessagesCall::receive_messages_proof
    );

    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        PangolinParachainMessagesToPangolin,
        PangolinParachainMessagesToPangolinReceiveMessagesDeliveryProofCallBuilder,
        relay_pangolin_parachain_client::runtime::Call::BridgePangolinMessages,
        relay_pangolin_parachain_client::runtime::BridgePangolinMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for PangolinParachainMessagesToPangolin {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangolinParachainChain;
        type TargetChain = PangolinChain;

        type SourceTransactionSignScheme = PangolinParachainChain;
        type TargetTransactionSignScheme = PangolinChain;

        type ReceiveMessagesProofCallBuilder =
            PangolinParachainMessagesToPangolinReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            PangolinParachainMessagesToPangolinReceiveMessagesDeliveryProofCallBuilder;

        type RelayStrategy = PangolinRelayStrategy;
    }
}
