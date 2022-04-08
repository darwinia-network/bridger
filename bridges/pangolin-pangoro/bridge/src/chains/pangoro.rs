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

        // todo: common relay strategy
        type RelayStrategy = PangoroRelayStrategy;
    }
}
