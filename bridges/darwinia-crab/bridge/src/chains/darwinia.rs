pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use std::time::Duration;

    use bp_messages::MessageNonce;
    use bp_runtime::ChainId;
    use frame_support::weights::Weight;
    use relay_substrate_client::Chain;
    use sp_version::RuntimeVersion;

    use client_crab::CrabChain;
    use client_darwinia::DarwiniaChain;

    use crate::traits::{ChainConst, CliChain};

    // === start const
    impl CliChain for DarwiniaChain {
        const RUNTIME_VERSION: RuntimeVersion = darwinia_runtime::VERSION;

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

    /// Description of Darwinia -> Crab finalized headers bridge.
    #[derive(Clone, Debug)]
    pub struct DarwiniaFinalityToCrab;

    impl SubstrateFinalitySyncPipeline for DarwiniaFinalityToCrab {
        type SourceChain = DarwiniaChain;
        type TargetChain = CrabChain;

        type SubmitFinalityProofCallBuilder = DirectSubmitFinalityProofCallBuilder<
            Self,
            crab_runtime::Runtime,
            crab_runtime::WithDarwiniaGrandpa,
        >;
        type TransactionSignScheme = CrabChain;
    }

    // === end
}

mod s2s_messages {
    use substrate_relay_helper::messages_lane::{
        DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
        SubstrateMessageLane,
    };

    use client_crab::CrabChain;
    use client_darwinia::{DarwiniaChain, DarwiniaRelayStrategy};

    #[derive(Clone, Debug)]
    pub struct DarwiniaMessagesToCrab;

    impl SubstrateMessageLane for DarwiniaMessagesToCrab {
        const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
        const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

        type SourceChain = DarwiniaChain;
        type TargetChain = CrabChain;

        type SourceTransactionSignScheme = DarwiniaChain;
        type TargetTransactionSignScheme = CrabChain;

        type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
            Self,
            crab_runtime::Runtime,
            crab_runtime::WithDarwiniaMessages,
        >;
        type ReceiveMessagesDeliveryProofCallBuilder =
            DirectReceiveMessagesDeliveryProofCallBuilder<
                Self,
                darwinia_runtime::Runtime,
                darwinia_runtime::WithCrabMessages,
            >;

        type RelayStrategy = DarwiniaRelayStrategy;
    }
}
