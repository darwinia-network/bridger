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

    use client_pangolin::PangolinChain;
    use client_pangoro::PangoroChain;

    use crate::traits::{ChainConst, CliChain};

    // === start const
    impl CliChain for PangoroChain {
        const RUNTIME_VERSION: RuntimeVersion = pangoro_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    pub struct PangoroChainConst;

    impl ChainConst for PangoroChainConst {
        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            drml_bridge_primitives::TO_PANGORO_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            drml_bridge_primitives::TO_PANGORO_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            drml_bridge_primitives::TO_PANGORO_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            drml_bridge_primitives::FROM_PANGORO_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            drml_bridge_primitives::FROM_PANGORO_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            drml_bridge_primitives::FROM_PANGORO_UNREWARDED_RELAYERS_STATE;
        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            drml_bridge_primitives::BEST_FINALIZED_PANGORO_HEADER_METHOD;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            drml_bridge_primitives::BEST_FINALIZED_PANGORO_HEADER_METHOD;
        const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
            drml_bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
        const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
            drml_bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
        const AVERAGE_BLOCK_INTERVAL: Duration = PangolinChain::AVERAGE_BLOCK_INTERVAL;
        const BRIDGE_CHAIN_ID: ChainId = drml_bridge_primitives::PANGORO_CHAIN_ID;
        const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
            drml_bridge_primitives::WITH_PANGOLIN_MESSAGES_PALLET_NAME;
        const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
            drml_bridge_primitives::WITH_PANGORO_MESSAGES_PALLET_NAME;
        const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
            drml_bridge_primitives::PAY_INBOUND_DISPATCH_FEE_WEIGHT;
        type SigningParams = drml_common_primitives::SigningParams;
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
    use codec::Encode;
    use sp_core::{Bytes, Pair};

    use client_pangolin::PangolinChain;
    use client_pangoro::{PangoroChain, PangoroRelayStrategy};
    use relay_substrate_client::{Client, SignParam, TransactionSignScheme, UnsignedTransaction};
    use substrate_relay_helper::messages_lane::{
        DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
        SubstrateMessageLane,
    };
    use substrate_relay_helper::messages_source::SubstrateMessagesSource;
    use substrate_relay_helper::messages_target::SubstrateMessagesTarget;

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
