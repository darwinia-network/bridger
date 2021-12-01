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

    use component_crab_s2s::CrabChain;
    use component_darwinia_s2s::DarwiniaChain;

    use crate::traits::{ChainConst, CliChain};

    // === start const
    impl CliChain for CrabChain {
        const RUNTIME_VERSION: RuntimeVersion = crab_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    pub struct CrabChainConst;

    impl ChainConst for CrabChainConst {
        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            darwinia_bridge_primitives::TO_CRAB_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            darwinia_bridge_primitives::TO_CRAB_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            darwinia_bridge_primitives::TO_CRAB_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            darwinia_bridge_primitives::FROM_CRAB_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            darwinia_bridge_primitives::FROM_CRAB_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            darwinia_bridge_primitives::FROM_CRAB_UNREWARDED_RELAYERS_STATE;
        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            darwinia_bridge_primitives::BEST_FINALIZED_CRAB_HEADER_METHOD;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            darwinia_bridge_primitives::BEST_FINALIZED_CRAB_HEADER_METHOD;
        const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
            darwinia_bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
        const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
            darwinia_bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
        const AVERAGE_BLOCK_INTERVAL: Duration = DarwiniaChain::AVERAGE_BLOCK_INTERVAL;
        const BRIDGE_CHAIN_ID: ChainId = darwinia_bridge_primitives::CRAB_CHAIN_ID;
        const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
            darwinia_bridge_primitives::WITH_DARWINIA_MESSAGES_PALLET_NAME;
        const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
            darwinia_bridge_primitives::WITH_CRAB_MESSAGES_PALLET_NAME;
        const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
            darwinia_bridge_primitives::PAY_INBOUND_DISPATCH_FEE_WEIGHT;
        type SigningParams = sp_core::sr25519::Pair;
    }

    // === end
}

mod s2s_headers {
    use bp_header_chain::justification::GrandpaJustification;
    use codec::Encode;
    use relay_substrate_client::{Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
    use sp_core::{Bytes, Pair};
    use substrate_relay_helper::finality_pipeline::{
        SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate,
    };

    use component_crab_s2s::CrabChain;
    use component_darwinia_s2s::DarwiniaChain;

    use crate::chains::crab::CrabChainConst;
    use crate::chains::darwinia::DarwiniaChainConst;
    use crate::traits::ChainConst;

    // === start crab headers to darwinia
    /// Crab-to-Darwinia finality sync pipeline.
    pub(crate) type FinalityPipelineCrabFinalityToDarwinia = SubstrateFinalityToSubstrate<
        CrabChain,
        DarwiniaChain,
        <DarwiniaChainConst as ChainConst>::SigningParams,
    >;

    #[derive(Clone, Debug)]
    pub struct CrabFinalityToDarwinia {
        finality_pipeline: FinalityPipelineCrabFinalityToDarwinia,
    }

    impl CrabFinalityToDarwinia {
        pub fn new(
            target_client: Client<DarwiniaChain>,
            target_sign: <DarwiniaChainConst as ChainConst>::SigningParams,
        ) -> Self {
            Self {
                finality_pipeline: FinalityPipelineCrabFinalityToDarwinia::new(
                    target_client,
                    target_sign,
                ),
            }
        }
    }

    impl SubstrateFinalitySyncPipeline for CrabFinalityToDarwinia {
        type FinalitySyncPipeline = FinalityPipelineCrabFinalityToDarwinia;

        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            CrabChainConst::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;

        type TargetChain = DarwiniaChain;

        fn transactions_author(&self) -> darwinia_common_primitives::AccountId {
            (*self.finality_pipeline.target_sign.public().as_array_ref()).into()
        }

        fn make_submit_finality_proof_transaction(
            &self,
            era: bp_runtime::TransactionEraOf<DarwiniaChain>,
            transaction_nonce: IndexOf<DarwiniaChain>,
            header: component_crab_s2s::SyncHeader,
            proof: GrandpaJustification<darwinia_common_primitives::Header>,
        ) -> Bytes {
            let call = darwinia_runtime::BridgeGrandpaCall::<
                darwinia_runtime::Runtime,
                darwinia_runtime::WithCrabGrandpa,
            >::submit_finality_proof(Box::new(header.into_inner()), proof)
            .into();

            let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
            let transaction = DarwiniaChain::sign_transaction(
                genesis_hash,
                &self.finality_pipeline.target_sign,
                era,
                UnsignedTransaction::new(call, transaction_nonce),
            );

            Bytes(transaction.encode())
        }
    }

    // === end
}

mod s2s_messages {
    use std::{ops::RangeInclusive, time::Duration};

    use bp_messages::MessageNonce;
    use bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
    use codec::Encode;
    use frame_support::dispatch::GetDispatchInfo;
    use frame_support::weights::Weight;
    use messages_relay::message_lane::MessageLane;
    use relay_substrate_client::{Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
    use relay_utils::metrics::MetricsParams;
    use sp_core::{Bytes, Pair};
    use substrate_relay_helper::messages_lane::{
        MessagesRelayParams, StandaloneMessagesMetrics, SubstrateMessageLane,
        SubstrateMessageLaneToSubstrate,
    };
    use substrate_relay_helper::messages_source::SubstrateMessagesSource;
    use substrate_relay_helper::messages_target::SubstrateMessagesTarget;

    use component_crab_s2s::{CrabChain, CrabRelayStrategy};
    use component_darwinia_s2s::DarwiniaChain;

    use crate::chains::crab::CrabChainConst;
    use crate::chains::darwinia::DarwiniaChainConst;
    use crate::traits::ChainConst;

    pub const SOURCE_NAME: &str = "crab";
    pub const TARGET_NAME: &str = "darwinia";

    /// Source-to-Target message lane.
    pub type MessageLaneCrabMessagesToDarwinia = SubstrateMessageLaneToSubstrate<
        CrabChain,
        <CrabChainConst as ChainConst>::SigningParams,
        DarwiniaChain,
        <DarwiniaChainConst as ChainConst>::SigningParams,
    >;

    #[derive(Clone)]
    pub struct CrabMessagesToDarwinia {
        message_lane: MessageLaneCrabMessagesToDarwinia,
    }

    impl SubstrateMessageLane for CrabMessagesToDarwinia {
        type MessageLane = MessageLaneCrabMessagesToDarwinia;

        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            DarwiniaChainConst::OUTBOUND_LANE_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            DarwiniaChainConst::OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            DarwiniaChainConst::OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;

        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            CrabChainConst::INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            CrabChainConst::INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            CrabChainConst::INBOUND_LANE_UNREWARDED_RELAYERS_STATE;

        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            CrabChainConst::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            DarwiniaChainConst::BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE;

        const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
            CrabChainConst::MESSAGE_PALLET_NAME_AT_SOURCE;
        const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
            CrabChainConst::MESSAGE_PALLET_NAME_AT_TARGET;
        const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
            CrabChainConst::PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN;

        type SourceChain = CrabChain;
        type TargetChain = DarwiniaChain;

        fn source_transactions_author(&self) -> darwinia_common_primitives::AccountId {
            (*self.message_lane.source_sign.public().as_array_ref()).into()
        }

        fn make_messages_receiving_proof_transaction(
            &self,
            transaction_nonce: IndexOf<CrabChain>,
            _generated_at_block: component_darwinia_s2s::HeaderId,
            proof: <Self::MessageLane as MessageLane>::MessagesReceivingProof,
        ) -> Bytes {
            let (relayers_state, proof) = proof;
            let call: crab_runtime::Call =
                crab_runtime::BridgeMessagesCall::receive_messages_delivery_proof::<
                    crab_runtime::Runtime,
                    crab_runtime::WithDarwiniaMessages,
                >(proof, relayers_state)
                .into();
            let call_weight = call.get_dispatch_info().weight;
            let genesis_hash = *self.message_lane.source_client.genesis_hash();
            let transaction = CrabChain::sign_transaction(
                genesis_hash,
                &self.message_lane.source_sign,
                relay_substrate_client::TransactionEra::immortal(),
                UnsignedTransaction::new(call, transaction_nonce),
            );
            log::trace!(
                target: "bridge",
                "Prepared {} -> {} confirmation transaction. Weight: {}/{}, size: {}/{}",
                TARGET_NAME,
                SOURCE_NAME,
                call_weight,
                darwinia_common_runtime::max_extrinsic_weight(),
                transaction.encode().len(),
                darwinia_common_runtime::max_extrinsic_size(),
            );
            Bytes(transaction.encode())
        }

        fn target_transactions_author(&self) -> darwinia_common_primitives::AccountId {
            (*self.message_lane.target_sign.public().as_array_ref()).into()
        }

        fn make_messages_delivery_transaction(
            &self,
            transaction_nonce: IndexOf<DarwiniaChain>,
            _generated_at_header: component_crab_s2s::HeaderId,
            _nonces: RangeInclusive<MessageNonce>,
            proof: <Self::MessageLane as MessageLane>::MessagesProof,
        ) -> Bytes {
            let (dispatch_weight, proof) = proof;
            let FromBridgedChainMessagesProof {
                ref nonces_start,
                ref nonces_end,
                ..
            } = proof;
            let messages_count = nonces_end - nonces_start + 1;
            let call: darwinia_runtime::Call =
                darwinia_runtime::BridgeMessagesCall::receive_messages_proof::<
                    darwinia_runtime::Runtime,
                    darwinia_runtime::WithCrabMessages,
                >(
                    self.message_lane.relayer_id_at_source.clone(),
                    proof,
                    messages_count as _,
                    dispatch_weight,
                )
                .into();
            let call_weight = call.get_dispatch_info().weight;
            let genesis_hash = *self.message_lane.target_client.genesis_hash();
            let transaction = DarwiniaChain::sign_transaction(
                genesis_hash,
                &self.message_lane.target_sign,
                relay_substrate_client::TransactionEra::immortal(),
                UnsignedTransaction::new(call, transaction_nonce),
            );
            log::trace!(
                target: "bridge",
                "Prepared {} -> {} delivery transaction. Weight: {}/{}, size: {}/{}",
                SOURCE_NAME,
                TARGET_NAME,
                call_weight,
                darwinia_common_runtime::max_extrinsic_weight(),
                transaction.encode().len(),
                darwinia_common_runtime::max_extrinsic_size(),
            );
            Bytes(transaction.encode())
        }
    }

    /// Source node as messages source.
    type CrabSourceClient = SubstrateMessagesSource<CrabMessagesToDarwinia>;

    /// Target node as messages target.
    type DarwiniaTargetClient = SubstrateMessagesTarget<CrabMessagesToDarwinia>;

    pub struct CrabMessagesToDarwiniaRunner;

    #[allow(non_snake_case)]
    impl CrabMessagesToDarwiniaRunner {
        pub async fn run(
            params: MessagesRelayParams<
                CrabChain,
                <CrabChainConst as ChainConst>::SigningParams,
                DarwiniaChain,
                <DarwiniaChainConst as ChainConst>::SigningParams,
                CrabRelayStrategy,
            >,
        ) -> anyhow::Result<()> {
            let stall_timeout = Duration::from_secs(5 * 60);
            let relayer_id_at_crab = (*params.source_sign.public().as_array_ref()).into();

            let lane_id = params.lane_id;
            let source_client = params.source_client;
            let lane = CrabMessagesToDarwinia {
                message_lane: SubstrateMessageLaneToSubstrate {
                    source_client: source_client.clone(),
                    source_sign: params.source_sign,
                    target_client: params.target_client.clone(),
                    target_sign: params.target_sign,
                    relayer_id_at_source: relayer_id_at_crab,
                },
            };

            // 2/3 is reserved for proofs and tx overhead
            let max_messages_size_in_single_batch =
                darwinia_common_runtime::max_extrinsic_size() / 3;
            let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
                substrate_relay_helper::messages_lane::select_delivery_transaction_limits::<
                    // todo: there can be change to special weight
                    pallet_bridge_messages::weights::RialtoWeight<crab_runtime::Runtime>,
                >(
                    darwinia_common_runtime::max_extrinsic_weight(),
                    DarwiniaChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
                );

            log::info!(
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
                    <CrabMessagesToDarwinia as SubstrateMessageLane>::MessageLane,
                >(&lane_id)),
                params.metrics_params,
                source_client.clone(),
            )?;
            messages_relay::message_lane_loop::run(
                messages_relay::message_lane_loop::Params {
                    lane: lane_id,
                    source_tick: CrabChainConst::AVERAGE_BLOCK_INTERVAL,
                    target_tick: DarwiniaChainConst::AVERAGE_BLOCK_INTERVAL,
                    reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
                    stall_timeout,
                    delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
                        max_unrewarded_relayer_entries_at_target:
                            DarwiniaChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
                        max_unconfirmed_nonces_at_target:
                            DarwiniaChainConst::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
                        max_messages_in_single_batch,
                        max_messages_weight_in_single_batch,
                        max_messages_size_in_single_batch,
                        relay_strategy: params.relay_strategy,
                    },
                },
                CrabSourceClient::new(
                    source_client.clone(),
                    lane.clone(),
                    lane_id,
                    params.target_to_source_headers_relay,
                ),
                DarwiniaTargetClient::new(
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
        }
    }

    /// Add standalone metrics for the Crab -> Darwinia messages loop.
    pub(crate) fn add_standalone_metrics(
        metrics_prefix: Option<String>,
        metrics_params: MetricsParams,
        source_client: Client<CrabChain>,
    ) -> anyhow::Result<(MetricsParams, StandaloneMessagesMetrics)> {
        substrate_relay_helper::messages_lane::add_standalone_metrics::<CrabMessagesToDarwinia>(
            metrics_prefix,
            metrics_params,
            source_client,
            Some(crate::chains::CRAB_ASSOCIATED_TOKEN_ID),
            Some(crate::chains::DARWINIA_ASSOCIATED_TOKEN_ID),
            Some((
                sp_core::storage::StorageKey(
                    crab_runtime::darwinia_messages::DarwiniaToCrabConversionRate::key().to_vec(),
                ),
                crab_runtime::darwinia_messages::INITIAL_DARWINIA_TO_CRAB_CONVERSION_RATE,
            )),
        )
    }
}
