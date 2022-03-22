pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use sp_version::RuntimeVersion;

    use client_crab::CrabChain;

    use crate::traits::CliChain;

    // === start const
    impl CliChain for CrabChain {
        const RUNTIME_VERSION: RuntimeVersion = crab_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use substrate_relay_helper::finality_pipeline::{
        DirectSubmitFinalityProofCallBuilder, SubstrateFinalitySyncPipeline,
    };

    use client_crab::CrabChain;
    use client_darwinia::DarwiniaChain;

    /// Description of Crab -> Darwinia finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct CrabFinalityToDarwinia;

    impl SubstrateFinalitySyncPipeline for CrabFinalityToDarwinia {
        type SourceChain = CrabChain;
        type TargetChain = DarwiniaChain;

        type SubmitFinalityProofCallBuilder = DirectSubmitFinalityProofCallBuilder<
            Self,
            darwinia_runtime::Runtime,
            darwinia_runtime::WithCrabGrandpa,
        >;
        type TransactionSignScheme = DarwiniaChain;
    }

    // === end
}

mod s2s_messages {
    use substrate_relay_helper::messages_lane::{
        DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
        SubstrateMessageLane,
    };

    use client_crab::{CrabChain, CrabRelayStrategy};
    use client_darwinia::DarwiniaChain;

    #[derive(Clone, Debug)]
    pub struct CrabMessagesToDarwinia;

    impl SubstrateMessageLane for CrabMessagesToDarwinia {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = CrabChain;
        type TargetChain = DarwiniaChain;

        type SourceTransactionSignScheme = CrabChain;
        type TargetTransactionSignScheme = DarwiniaChain;

        type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
            Self,
            darwinia_runtime::Runtime,
            darwinia_runtime::WithCrabMessages,
        >;
        type ReceiveMessagesDeliveryProofCallBuilder =
            DirectReceiveMessagesDeliveryProofCallBuilder<
                Self,
                crab_runtime::Runtime,
                crab_runtime::WithDarwiniaMessages,
            >;

        type RelayStrategy = CrabRelayStrategy;
    }
}
