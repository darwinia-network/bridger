pub use s2s_const::*;
pub use s2s_headers::*;
pub use s2s_messages::*;

mod s2s_const {
    use std::time::Duration;

    use bp_messages::MessageNonce;
    use bp_runtime::ChainId;
    use relay_substrate_client::Chain;
    use sp_version::RuntimeVersion;

    use component_pangolin_s2s::PangolinChain;
    use component_pangoro_s2s::PangoroChain;

    use crate::traits::{ChainConst, CliChain};

    // === start const
    impl CliChain for PangoroChain {
        const RUNTIME_VERSION: RuntimeVersion = pangoro_runtime::VERSION;

        type KeyPair = sp_core::sr25519::Pair;
    }

    pub struct PangoroChainConst;

    impl ChainConst for PangoroChainConst {
        const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
            bridge_primitives::TO_PANGORO_MESSAGE_DETAILS_METHOD;
        const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
            bridge_primitives::TO_PANGORO_LATEST_GENERATED_NONCE_METHOD;
        const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            bridge_primitives::TO_PANGORO_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
            bridge_primitives::FROM_PANGORO_LATEST_RECEIVED_NONCE_METHOD;
        const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
            bridge_primitives::FROM_PANGORO_LATEST_CONFIRMED_NONCE_METHOD;
        const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
            bridge_primitives::FROM_PANGORO_UNREWARDED_RELAYERS_STATE;
        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            bridge_primitives::BEST_FINALIZED_PANGORO_HEADER_METHOD;
        const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
            bridge_primitives::BEST_FINALIZED_PANGORO_HEADER_METHOD;
        const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
            bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
        const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
            bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
        const AVERAGE_BLOCK_INTERVAL: Duration = PangolinChain::AVERAGE_BLOCK_INTERVAL;
        const BRIDGE_CHAIN_ID: ChainId = bridge_primitives::PANGORO_CHAIN_ID;
        type SigningParams = common_primitives::SigningParams;
    }

    // === end
}

mod s2s_headers {
    use bp_header_chain::justification::GrandpaJustification;
    use codec::Encode;
    use relay_substrate_client::{Chain, Client, TransactionSignScheme};
    use sp_core::{Bytes, Pair};
    use substrate_relay_helper::finality_pipeline::{
        SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate,
    };

    use component_pangolin_s2s::PangolinChain;
    use component_pangoro_s2s::PangoroChain;

    use crate::chains::pangolin::PangolinChainConst;
    use crate::chains::pangoro::PangoroChainConst;
    use crate::traits::ChainConst;

    // === start pangolin headers to pangoro
    /// Pangoro-to-Pangolin finality sync pipeline.
    pub(crate) type FinalityPipelinePangoroFinalityToPangolin = SubstrateFinalityToSubstrate<
        PangoroChain,
        PangolinChain,
        <PangolinChainConst as ChainConst>::SigningParams,
    >;

    #[derive(Clone, Debug)]
    pub struct PangoroFinalityToPangolin {
        finality_pipeline: FinalityPipelinePangoroFinalityToPangolin,
    }

    impl PangoroFinalityToPangolin {
        pub fn new(
            target_client: Client<PangolinChain>,
            target_sign: <PangolinChainConst as ChainConst>::SigningParams,
        ) -> Self {
            Self {
                finality_pipeline: FinalityPipelinePangoroFinalityToPangolin::new(
                    target_client,
                    target_sign,
                ),
            }
        }
    }

    impl SubstrateFinalitySyncPipeline for PangoroFinalityToPangolin {
        type FinalitySyncPipeline = FinalityPipelinePangoroFinalityToPangolin;

        const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
            PangoroChainConst::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;

        type TargetChain = PangolinChain;

        fn transactions_author(&self) -> common_primitives::AccountId {
            (*self.finality_pipeline.target_sign.public().as_array_ref()).into()
        }

        fn make_submit_finality_proof_transaction(
            &self,
            transaction_nonce: <PangolinChain as Chain>::Index,
            header: component_pangoro_s2s::SyncHeader,
            proof: GrandpaJustification<common_primitives::Header>,
        ) -> Bytes {
            let call = pangolin_runtime::BridgeGrandpaCall::<
                pangolin_runtime::Runtime,
                pangolin_runtime::WithPangoroGrandpa,
            >::submit_finality_proof(header.into_inner(), proof)
            .into();

            let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
            let transaction = PangolinChain::sign_transaction(
                genesis_hash,
                &self.finality_pipeline.target_sign,
                transaction_nonce,
                call,
            );

            Bytes(transaction.encode())
        }
    }

    // === end
}

mod s2s_messages {}
