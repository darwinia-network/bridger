pub use pangolin_runtime;

use bp_header_chain::justification::GrandpaJustification;
use bp_messages::MessageNonce;
use bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use codec::Encode;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::sp_runtime::FixedU128;
use messages_relay::message_lane::MessageLane;
use millau_runtime::{
	BridgeGrandpaPangolinCall, MessagesCall as TargetChainRuntimeMessagesCall, WithPangolinGrandpaInstance,
	WithPangolinMessagesInstance,
};
use pangolin_runtime::bridge::s2s::{
	millau_messages::MillauToPangolinConversionRate, millau_messages::INITIAL_MILLAU_TO_PANGOLIN_CONVERSION_RATE,
	MessagesCall as SourceChainRuntimeMessagesCall, WithMillauMessagesInstance,
};
use pangolin_runtime_params::s2s as s2s_params;
use pangolin_runtime_params::system as pangolin_params_system;
use relay_millau_client::Millau as MillauRelayChain;
use relay_pangolin_client::PangolinRelayChain;
use relay_substrate_client::{
	metrics::{FloatStorageValueMetric, StorageProofOverheadMetric},
	Chain as RelaySubstrateClientChain, TransactionSignScheme,
};
use sp_core::{Bytes, Pair};
use sp_version::RuntimeVersion;
use std::{ops::RangeInclusive, time::Duration};

use crate::types::s2s::{
	finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate},
	messages_lane::{
		select_delivery_transaction_limits, MessagesRelayParams, SubstrateMessageLane, SubstrateMessageLaneToSubstrate,
	},
	messages_source::SubstrateMessagesSource,
	messages_target::SubstrateMessagesTarget,
};
use crate::*;

pub struct PangolinChainConst;

impl ChainConst for PangolinChainConst {
	const OUTBOUND_LANE_MESSAGES_DISPATCH_WEIGHT_METHOD: &'static str =
		s2s_params::TO_PANGOLIN_MESSAGES_DISPATCH_WEIGHT_METHOD;
	const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
		s2s_params::TO_PANGOLIN_LATEST_GENERATED_NONCE_METHOD;
	const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		s2s_params::TO_PANGOLIN_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		s2s_params::FROM_PANGOLIN_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
		s2s_params::FROM_PANGOLIN_LATEST_CONFIRMED_NONCE_METHOD;
	const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str = s2s_params::FROM_PANGOLIN_UNREWARDED_RELAYERS_STATE;
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = s2s_params::BEST_FINALIZED_PANGOLIN_HEADER_METHOD;
	const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str = s2s_params::BEST_FINALIZED_PANGOLIN_HEADER_METHOD;
	const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
		s2s_params::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
	const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce = s2s_params::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
	const AVERAGE_BLOCK_INTERVAL: Duration = PangolinRelayChain::AVERAGE_BLOCK_INTERVAL;
}

declare_cli_chain!(PangolinRelayChain, pangolin_runtime);

declare_relay_chain!(Pangolin, {
	const CHAIN_NAME: &'static str = "pangolin";
	const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;
	type Runtime = pangolin_runtime::Runtime;
	type HeaderId = relay_pangolin_client::HeaderId;
	type Chain = relay_pangolin_client::PangolinRelayChain;
	type SigningParams = relay_pangolin_client::SigningParams;
	type SyncHeader = relay_pangolin_client::SyncHeader;
});

declare_relay_headers!(
	Pangolin,
	Millau,
	PangolinRelayChain,
	MillauRelayChain,
	relay_pangolin_client,
	PangolinChainConst,
	drml_primitives,
	bp_millau,
	millau_runtime,
	BridgeGrandpaPangolinCall,
	WithPangolinGrandpaInstance,
);

declare_relay_messages!(
	Pangolin,
	Millau,
	PangolinRelayChain,
	MillauRelayChain,
	relay_pangolin_client,
	relay_millau_client,
	PangolinChainConst,
	MillauChainConst,
	drml_primitives,
	bp_millau,
	pangolin_runtime,
	millau_runtime,
	SourceChainRuntimeMessagesCall,
	TargetChainRuntimeMessagesCall,
	pangolin_params_system,
	bp_millau,
	WithPangolinMessagesInstance,
	WithMillauMessagesInstance,
	MillauToPangolinConversionRate,
	INITIAL_MILLAU_TO_PANGOLIN_CONVERSION_RATE,
);
