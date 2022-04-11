pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use relay_pangolin_client::PangolinChain;
    use sp_version::RuntimeVersion;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinChain {
        const RUNTIME_VERSION: RuntimeVersion = bp_pangolin::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use relay_pangolin_client::PangolinChain;
    use relay_pangoro_client::PangoroChain;
    use substrate_relay_helper::finality_pipeline::SubstrateFinalitySyncPipeline;

    /// Description of Pangolin -> Pangoro finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct PangolinFinalityToPangoro;

    substrate_relay_helper::generate_mocked_submit_finality_proof_call_builder!(
        PangolinFinalityToPangoro,
        PangolinFinalityToPangoroCallBuilder,
        relay_pangoro_client::runtime::Call::BridgePangolinGrandpa,
        relay_pangoro_client::runtime::BridgePangolinGrandpaCall::submit_finality_proof
    );

    impl SubstrateFinalitySyncPipeline for PangolinFinalityToPangoro {
        type SourceChain = PangolinChain;
        type TargetChain = PangoroChain;

        type SubmitFinalityProofCallBuilder = PangolinFinalityToPangoroCallBuilder;
        type TransactionSignScheme = PangoroChain;
    }

    // === end
}

mod s2s_messages {
    use frame_support::weights::Weight;
    use relay_pangolin_client::PangolinChain;
    use relay_pangoro_client::PangoroChain;
    use substrate_relay_helper::messages_lane::SubstrateMessageLane;

    #[derive(Clone, Debug)]
    pub struct PangolinMessagesToPangoro;

    substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
        PangolinMessagesToPangoro,
        PangolinMessagesToPangoroReceiveMessagesProofCallBuilder,
        relay_pangoro_client::runtime::Call::BridgePangolinMessages,
        relay_pangoro_client::runtime::BridgePangolinMessagesCall::receive_messages_proof
    );
    substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
        PangolinMessagesToPangoro,
        PangolinMessagesToPangoroReceiveMessagesDeliveryProofCallBuilder,
        relay_pangolin_client::runtime::Call::BridgePangoroMessages,
        relay_pangolin_client::runtime::BridgePangoroMessagesCall::receive_messages_delivery_proof
    );

    impl SubstrateMessageLane for PangolinMessagesToPangoro {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangolinChain;
        type TargetChain = PangoroChain;

        type SourceTransactionSignScheme = PangolinChain;
        type TargetTransactionSignScheme = PangoroChain;

        type ReceiveMessagesProofCallBuilder =
            PangolinMessagesToPangoroReceiveMessagesProofCallBuilder;
        type ReceiveMessagesDeliveryProofCallBuilder =
            PangolinMessagesToPangoroReceiveMessagesDeliveryProofCallBuilder;

        // todo: common relay strategy
        type RelayStrategy = PangolinRelayStrategy;
    }
}

mod s2s_feemarket {}
