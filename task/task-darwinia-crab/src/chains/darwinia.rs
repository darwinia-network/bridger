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

    use component_darwinia_s2s::DarwiniaChain;

    use crate::traits::{ChainConst, CliChain};

    // === start const
    impl CliChain for DarwiniaChain {
        const RUNTIME_VERSION: RuntimeVersion = darwinia_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    pub struct DarwiniaChainConst;

    impl ChainConst for DarwiniaChainConst {
        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            bridge_primitives::TO_DARWINIA_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            bridge_primitives::TO_DARWINIA_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            bridge_primitives::TO_DARWINIA_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            bridge_primitives::FROM_DARWINIA_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            bridge_primitives::FROM_DARWINIA_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            bridge_primitives::FROM_DARWINIA_UNREWARDED_RELAYERS_STATE;
        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            bridge_primitives::BEST_FINALIZED_DARWINIA_HEADER_METHOD;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            bridge_primitives::BEST_FINALIZED_DARWINIA_HEADER_METHOD;
        const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
            bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
        const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
            bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
        const AVERAGE_BLOCK_INTERVAL: Duration = DarwiniaChain::AVERAGE_BLOCK_INTERVAL;
        const BRIDGE_CHAIN_ID: ChainId = bridge_primitives::DARWINIA_CHAIN_ID;
        const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
            bridge_primitives::WITH_CRAB_MESSAGES_PALLET_NAME;
        const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
            bridge_primitives::WITH_DARWINIA_MESSAGES_PALLET_NAME;
        const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
            bridge_primitives::PAY_INBOUND_DISPATCH_FEE_WEIGHT;
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

    // === start pangolin headers to pangoro
    /// Pangolin-to-Pangoro finality sync pipeline.
    pub(crate) type FinalityPipelineDarwiniaFinalityToCrab = SubstrateFinalityToSubstrate<
        DarwiniaChain,
        CrabChain,
        <CrabChainConst as ChainConst>::SigningParams,
    >;

    #[derive(Clone, Debug)]
    pub struct DarwiniaFinalityToCrab {
        finality_pipeline: FinalityPipelineDarwiniaFinalityToCrab,
    }

    impl DarwiniaFinalityToCrab {
        pub fn new(
            target_client: Client<CrabChain>,
            target_sign: <CrabChainConst as ChainConst>::SigningParams,
        ) -> Self {
            Self {
                finality_pipeline: FinalityPipelineDarwiniaFinalityToCrab::new(
                    target_client,
                    target_sign,
                ),
            }
        }
    }

    impl SubstrateFinalitySyncPipeline for DarwiniaFinalityToCrab {
        type FinalitySyncPipeline = FinalityPipelineDarwiniaFinalityToCrab;

        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            DarwiniaChainConst::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;

        type TargetChain = CrabChain;

        fn transactions_author(&self) -> common_primitives::AccountId {
            (*self.finality_pipeline.target_sign.public().as_array_ref()).into()
        }

        fn make_submit_finality_proof_transaction(
            &self,
            era: bp_runtime::TransactionEraOf<CrabChain>,
            transaction_nonce: IndexOf<CrabChain>,
            header: component_darwinia_s2s::SyncHeader,
            proof: GrandpaJustification<common_primitives::Header>,
        ) -> Bytes {
            let call = crab_runtime::BridgeGrandpaCall::<
                crab_runtime::Runtime,
                crab_runtime::WithDarwiniaGrandpa,
            >::submit_finality_proof(Box::new(header.into_inner()), proof)
            .into();

            let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
            let transaction = CrabChain::sign_transaction(
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

    use component_crab_s2s::CrabChain;
    use component_darwinia_s2s::{DarwiniaChain, DarwiniaRelayStrategy};

    use crate::chains::crab::CrabChainConst;
    use crate::chains::darwinia::DarwiniaChainConst;
    use crate::traits::ChainConst;

    pub const SOURCE_NAME: &str = "darwinia";
    pub const TARGET_NAME: &str = "crab";

    /// Source-to-Target message lane.
    pub type MessageLaneDarwiniaMessagesToPangoro = SubstrateMessageLaneToSubstrate<
        DarwiniaChain,
        <DarwiniaChainConst as ChainConst>::SigningParams,
        CrabChain,
        <CrabChainConst as ChainConst>::SigningParams,
    >;

    #[derive(Clone)]
    pub struct DarwiniaMessagesToPangoro {
        message_lane: MessageLaneDarwiniaMessagesToPangoro,
    }

    impl SubstrateMessageLane for DarwiniaMessagesToPangoro {
        type MessageLane = MessageLaneDarwiniaMessagesToPangoro;

        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            CrabChainConst::OUTBOUND_LANE_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            CrabChainConst::OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            CrabChainConst::OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;

        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            DarwiniaChainConst::INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            DarwiniaChainConst::INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            DarwiniaChainConst::INBOUND_LANE_UNREWARDED_RELAYERS_STATE;

        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            DarwiniaChainConst::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            CrabChainConst::BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE;

        const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
            DarwiniaChainConst::MESSAGE_PALLET_NAME_AT_SOURCE;
        const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
            DarwiniaChainConst::MESSAGE_PALLET_NAME_AT_TARGET;
        const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
            DarwiniaChainConst::PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN;

        type SourceChain = DarwiniaChain;
        type TargetChain = CrabChain;

        fn source_transactions_author(&self) -> common_primitives::AccountId {
            (*self.message_lane.source_sign.public().as_array_ref()).into()
        }

        fn make_messages_receiving_proof_transaction(
            &self,
            transaction_nonce: IndexOf<DarwiniaChain>,
            _generated_at_block: component_crab_s2s::HeaderId,
            proof: <Self::MessageLane as MessageLane>::MessagesReceivingProof,
        ) -> Bytes {
            let (relayers_state, proof) = proof;
            let call: darwinia_runtime::Call =
                darwinia_runtime::BridgeMessagesCall::receive_messages_delivery_proof::<
                    darwinia_runtime::Runtime,
                    darwinia_runtime::WithCrabMessages,
                >(proof, relayers_state)
                .into();
            let call_weight = call.get_dispatch_info().weight;
            let genesis_hash = *self.message_lane.source_client.genesis_hash();
            let transaction = DarwiniaChain::sign_transaction(
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
                darwinia_runtime_common::max_extrinsic_weight(),
                transaction.encode().len(),
                darwinia_runtime_common::max_extrinsic_size(),
            );
            Bytes(transaction.encode())
        }

        fn target_transactions_author(&self) -> common_primitives::AccountId {
            (*self.message_lane.target_sign.public().as_array_ref()).into()
        }

        fn make_messages_delivery_transaction(
            &self,
            transaction_nonce: IndexOf<CrabChain>,
            _generated_at_header: component_darwinia_s2s::HeaderId,
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
            let call: crab_runtime::Call =
                crab_runtime::BridgeMessagesCall::receive_messages_proof::<
                    crab_runtime::Runtime,
                    crab_runtime::WithDarwiniaMessages,
                >(
                    self.message_lane.relayer_id_at_source.clone(),
                    proof,
                    messages_count as _,
                    dispatch_weight,
                )
                .into();
            let call_weight = call.get_dispatch_info().weight;
            let genesis_hash = *self.message_lane.target_client.genesis_hash();
            let transaction = CrabChain::sign_transaction(
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
                darwinia_runtime_common::max_extrinsic_weight(),
                transaction.encode().len(),
                darwinia_runtime_common::max_extrinsic_size(),
            );
            Bytes(transaction.encode())
        }
    }

    /// Source node as messages source.
    type DarwiniaSourceClient = SubstrateMessagesSource<DarwiniaMessagesToPangoro>;

    /// Target node as messages target.
    type CrabTargetClient = SubstrateMessagesTarget<DarwiniaMessagesToPangoro>;

    pub struct DarwiniaMessagesToPangoroRunner;

    #[allow(non_snake_case)]
    impl DarwiniaMessagesToPangoroRunner {
        pub async fn run(
            params: MessagesRelayParams<
                DarwiniaChain,
                <DarwiniaChainConst as ChainConst>::SigningParams,
                CrabChain,
                <CrabChainConst as ChainConst>::SigningParams,
                DarwiniaRelayStrategy,
            >,
        ) -> anyhow::Result<()> {
            let stall_timeout = Duration::from_secs(5 * 60);
            let relayer_id_at_darwinia = (*params.source_sign.public().as_array_ref()).into();

            let lane_id = params.lane_id;
            let source_client = params.source_client;
            let lane = DarwiniaMessagesToPangoro {
                message_lane: SubstrateMessageLaneToSubstrate {
                    source_client: source_client.clone(),
                    source_sign: params.source_sign,
                    target_client: params.target_client.clone(),
                    target_sign: params.target_sign,
                    relayer_id_at_source: relayer_id_at_darwinia,
                },
            };

            // 2/3 is reserved for proofs and tx overhead
            let max_messages_size_in_single_batch =
                darwinia_runtime_common::max_extrinsic_size() / 3;
            let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
                substrate_relay_helper::messages_lane::select_delivery_transaction_limits::<
                    // todo: there can be change to special weight
                    pallet_bridge_messages::weights::RialtoWeight<darwinia_runtime::Runtime>,
                >(
                    darwinia_runtime_common::max_extrinsic_weight(),
                    CrabChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
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
                    <DarwiniaMessagesToPangoro as SubstrateMessageLane>::MessageLane,
                >(&lane_id)),
                params.metrics_params,
                source_client.clone(),
            )?;
            messages_relay::message_lane_loop::run(
                messages_relay::message_lane_loop::Params {
                    lane: lane_id,
                    source_tick: DarwiniaChainConst::AVERAGE_BLOCK_INTERVAL,
                    target_tick: CrabChainConst::AVERAGE_BLOCK_INTERVAL,
                    reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
                    stall_timeout,
                    delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
                        max_unrewarded_relayer_entries_at_target:
                            CrabChainConst::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
                        max_unconfirmed_nonces_at_target:
                            CrabChainConst::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
                        max_messages_in_single_batch,
                        max_messages_weight_in_single_batch,
                        max_messages_size_in_single_batch,
                        relay_strategy: params.relay_strategy,
                    },
                },
                DarwiniaSourceClient::new(
                    source_client.clone(),
                    lane.clone(),
                    lane_id,
                    params.target_to_source_headers_relay,
                ),
                CrabTargetClient::new(
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

    /// Add standalone metrics for the Pangolin -> Pangoro messages loop.
    pub(crate) fn add_standalone_metrics(
        metrics_prefix: Option<String>,
        metrics_params: MetricsParams,
        source_client: Client<DarwiniaChain>,
    ) -> anyhow::Result<(MetricsParams, StandaloneMessagesMetrics)> {
        substrate_relay_helper::messages_lane::add_standalone_metrics::<DarwiniaMessagesToPangoro>(
            metrics_prefix,
            metrics_params,
            source_client,
            Some(crate::chains::DARWINIA_ASSOCIATED_TOKEN_ID),
            Some(crate::chains::CRAB_ASSOCIATED_TOKEN_ID),
            Some((
                sp_core::storage::StorageKey(
                    darwinia_runtime::crab_messages::CrabToDarwiniaConversionRate::key().to_vec(),
                ),
                darwinia_runtime::crab_messages::INITIAL_CRAB_TO_DARWINIA_CONVERSION_RATE,
            )),
        )
    }

    /// Update Pangolin -> Pangoro conversion rate, stored in Rialto runtime storage.
    pub(crate) async fn update_pangolin_to_pangoro_conversion_rate(
        client: Client<CrabChain>,
        signer: <CrabChain as TransactionSignScheme>::AccountKeyPair,
        updated_rate: f64,
    ) -> anyhow::Result<()> {
        let genesis_hash = *client.genesis_hash();
        let signer_id = (*signer.public().as_array_ref()).into();
        client
            .submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
                Bytes(
                    CrabChain::sign_transaction(
                        genesis_hash,
                        &signer,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            crab_runtime::BridgeMessagesCall::update_pallet_parameter(
                                crab_runtime::darwinia_messages::CrabToDarwiniaMessagesParameter::DarwiniaToCrabConversionRate(
                                    sp_runtime::FixedU128::from_float(updated_rate),
                                ),
                            )
                                .into(),
                            transaction_nonce,
                        ),
                    )
                        .encode(),
                )
            })
            .await
            .map(drop)
            .map_err(|err| anyhow::format_err!("{:?}", err))
    }
}
