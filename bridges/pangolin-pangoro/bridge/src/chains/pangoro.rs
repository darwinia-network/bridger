pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_pangoro::PangoroChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangoroChain {
        const RUNTIME_VERSION: RuntimeVersion = pangoro_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use client_pangolin::PangolinChain;
    use client_pangoro::PangoroChain;
    use substrate_relay_helper::finality_pipeline::{
        DirectSubmitFinalityProofCallBuilder, SubstrateFinalitySyncPipeline,
    };

    /// Description of Pangoro -> Pangolin finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct PangoroFinalityToPangolin;

    impl SubstrateFinalitySyncPipeline for PangoroFinalityToPangolin {
        type SourceChain = PangoroChain;
        type TargetChain = PangolinChain;

        type SubmitFinalityProofCallBuilder = DirectSubmitFinalityProofCallBuilder<
            Self,
            pangolin_runtime::Runtime,
            pangolin_runtime::WithPangoroGrandpa,
        >;
        type TransactionSignScheme = PangolinChain;
    }
    // === end
}

mod s2s_messages {

    use client_pangolin::PangolinChain;
    use client_pangoro::{PangoroChain, PangoroRelayStrategy};
    use substrate_relay_helper::messages_lane::{
        DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
        SubstrateMessageLane,
    };

    #[derive(Clone, Debug)]
    pub struct PangoroMessagesToPangolin;

    impl SubstrateMessageLane for PangoroMessagesToPangolin {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangoroChain;
        type TargetChain = PangolinChain;

        type SourceTransactionSignScheme = PangoroChain;
        type TargetTransactionSignScheme = PangolinChain;

        type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
            Self,
            pangolin_runtime::Runtime,
            pangolin_runtime::WithPangoroMessages,
        >;
        type ReceiveMessagesDeliveryProofCallBuilder =
            DirectReceiveMessagesDeliveryProofCallBuilder<
                Self,
                pangoro_runtime::Runtime,
                pangoro_runtime::WithPangolinMessages,
            >;

        type RelayStrategy = PangoroRelayStrategy;
    }
}
