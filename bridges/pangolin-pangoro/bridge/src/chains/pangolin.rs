pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_pangolin::PangolinChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for PangolinChain {
        const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;

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

    /// Description of Pangolin -> Pangoro finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct PangolinFinalityToPangoro;

    impl SubstrateFinalitySyncPipeline for PangolinFinalityToPangoro {
        type SourceChain = PangolinChain;
        type TargetChain = PangoroChain;

        type SubmitFinalityProofCallBuilder = DirectSubmitFinalityProofCallBuilder<
            Self,
            pangoro_runtime::Runtime,
            pangoro_runtime::WithPangolinGrandpa,
        >;
        type TransactionSignScheme = PangoroChain;
    }

    // === end
}

mod s2s_messages {
    use client_pangolin::{PangolinChain, PangolinRelayStrategy};
    use client_pangoro::PangoroChain;
    use substrate_relay_helper::messages_lane::{
        DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
        SubstrateMessageLane,
    };

    #[derive(Clone, Debug)]
    pub struct PangolinMessagesToPangoro;

    impl SubstrateMessageLane for PangolinMessagesToPangoro {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = PangolinChain;
        type TargetChain = PangoroChain;

        type SourceTransactionSignScheme = PangolinChain;
        type TargetTransactionSignScheme = PangoroChain;

        type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
            Self,
            pangoro_runtime::Runtime,
            pangoro_runtime::WithPangolinMessages,
        >;
        type ReceiveMessagesDeliveryProofCallBuilder =
            DirectReceiveMessagesDeliveryProofCallBuilder<
                Self,
                pangolin_runtime::Runtime,
                pangolin_runtime::WithPangoroMessages,
            >;

        type RelayStrategy = PangolinRelayStrategy;
    }
}
