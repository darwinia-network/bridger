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

    /// Source node as messages source.
    type PangoroSourceClient = SubstrateMessagesSource<PangoroMessagesToPangolin>;

    /// Target node as messages target.
    type PangolinTargetClient = SubstrateMessagesTarget<PangoroMessagesToPangolin>;

    pub struct PangoroMessagesToPangolinRunner;

    #[allow(non_snake_case)]
    impl PangoroMessagesToPangolinRunner {
        pub async fn run(
            params: MessagesRelayParams<
                PangoroChain,
                <PangoroChainConst as ChainConst>::SigningParams,
                PangolinChain,
                <PangolinChainConst as ChainConst>::SigningParams,
                PangoroRelayStrategy,
            >,
        ) -> color_eyre::Result<()> {
            let stall_timeout = Duration::from_secs(5 * 60);
            let relayer_id_at_Pangoro = (*params.source_sign.public().as_array_ref()).into();

            let lane_id = params.lane_id;
            let source_client = params.source_client;
            let lane = PangoroMessagesToPangolin {
                message_lane: SubstrateMessageLaneToSubstrate {
                    source_client: source_client.clone(),
                    source_sign: params.source_sign,
                    target_client: params.target_client.clone(),
                    target_sign: params.target_sign,
                    relayer_id_at_source: relayer_id_at_Pangoro,
                },
            };

            // 2/3 is reserved for proofs and tx overhead
            let max_messages_size_in_single_batch = common_runtime::max_extrinsic_size() / 3;
            let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
                substrate_relay_helper::messages_lane::select_delivery_transaction_limits::<
                    // todo: there can be change to special weight
                    pallet_bridge_messages::weights::RialtoWeight<pangoro_runtime::Runtime>,
                >(
                    common_runtime::max_extrinsic_weight(),
                    PangolinChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
                );

            tracing::info!(
                target: "bridge",
                "Starting {} -> {} messages relay.\n\t\
                {} relayer account id: {:?}\n\t\
                Max messages in single transaction: {}\n\t\
                Max messages size in single transaction: {}\n\t\
                Max messages weight in single transaction: {}",
                SOURCE_NAME,
                TARGET_NAME,
                SOURCE_NAME,
                lane.message_lane.relayer_id_at_source,
                max_messages_in_single_batch,
                max_messages_size_in_single_batch,
                max_messages_weight_in_single_batch,
            );

            let (metrics_params, metrics_values) = add_standalone_metrics(
                Some(messages_relay::message_lane_loop::metrics_prefix::<
                    <PangoroMessagesToPangolin as SubstrateMessageLane>::MessageLane,
                >(&lane_id)),
                params.metrics_params,
                source_client.clone(),
            )?;
            messages_relay::message_lane_loop::run(
                messages_relay::message_lane_loop::Params {
                    lane: lane_id,
                    source_tick: PangoroChainConst::AVERAGE_BLOCK_INTERVAL,
                    target_tick: PangolinChainConst::AVERAGE_BLOCK_INTERVAL,
                    reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
                    stall_timeout,
                    delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
                        max_unrewarded_relayer_entries_at_target:
                            PangolinChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
                        max_unconfirmed_nonces_at_target:
                            PangolinChainConst::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
                        max_messages_in_single_batch,
                        max_messages_weight_in_single_batch,
                        max_messages_size_in_single_batch,
                        relay_strategy: params.relay_strategy,
                    },
                },
                PangoroSourceClient::new(
                    source_client.clone(),
                    lane.clone(),
                    lane_id,
                    params.target_to_source_headers_relay,
                ),
                PangolinTargetClient::new(
                    params.target_client,
                    lane,
                    lane_id,
                    metrics_values,
                    params.source_to_target_headers_relay,
                ),
                metrics_params,
                futures::future::pending(),
            )
            .await
            .map_err(|e| BridgerError::Custom(format!("{:?}", e)).into())
        }
    }

    /// Add standalone metrics for the Pangoro -> Pangolin messages loop.
    pub(crate) fn add_standalone_metrics(
        metrics_prefix: Option<String>,
        metrics_params: MetricsParams,
        source_client: Client<PangoroChain>,
    ) -> color_eyre::Result<(MetricsParams, StandaloneMessagesMetrics)> {
        substrate_relay_helper::messages_lane::add_standalone_metrics::<PangoroMessagesToPangolin>(
            metrics_prefix,
            metrics_params,
            source_client,
            Some(crate::chains::PANGORO_ASSOCIATED_TOKEN_ID),
            Some(crate::chains::PANGOLIN_ASSOCIATED_TOKEN_ID),
            Some((
                sp_core::storage::StorageKey(
                    pangoro_runtime::pangolin_messages::PangolinToPangoroConversionRate::key()
                        .to_vec(),
                ),
                pangoro_runtime::pangolin_messages::INITIAL_PANGOLIN_TO_PANGORO_CONVERSION_RATE,
            )),
        )
        .map_err(|e| BridgerError::Custom(format!("{:?}", e)).into())
    }
}
