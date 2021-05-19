use bp_header_chain::justification::GrandpaJustification;
use bp_messages::MessageNonce;
use bp_runtime::ChainId;
use bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use codec::Encode;
use frame_support::dispatch::GetDispatchInfo;
use messages_relay::message_lane::MessageLane;
use millau_runtime::{
	BridgeGrandpaCall as BridgeGrandpaPangolinCall, BridgeMessagesCall as TargetChainRuntimeMessagesCall,
	WithPangolinGrandpa as WithPangolinGrandpaInstance, WithPangolinMessages as WithPangolinMessagesInstance,
};
use pangolin_bridge_relay_client_definition::PangolinChain;
use pangolin_runtime::{
	millau_messages::MillauToPangolinConversionRate, millau_messages::INITIAL_MILLAU_TO_PANGOLIN_CONVERSION_RATE,
	BridgeMessagesCall as SourceChainRuntimeMessagesCall, WithMillauMessages as WithMillauMessagesInstance,
};
use relay_millau_client::Millau as MillauChain;
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
		pangolin_bridge_primitives::TO_PANGOLIN_MESSAGES_DISPATCH_WEIGHT_METHOD;
	const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
		pangolin_bridge_primitives::TO_PANGOLIN_LATEST_GENERATED_NONCE_METHOD;
	const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		pangolin_bridge_primitives::TO_PANGOLIN_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		pangolin_bridge_primitives::FROM_PANGOLIN_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
		pangolin_bridge_primitives::FROM_PANGOLIN_LATEST_CONFIRMED_NONCE_METHOD;
	const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
		pangolin_bridge_primitives::FROM_PANGOLIN_UNREWARDED_RELAYERS_STATE;
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		pangolin_bridge_primitives::BEST_FINALIZED_PANGOLIN_HEADER_METHOD;
	const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
		pangolin_bridge_primitives::BEST_FINALIZED_PANGOLIN_HEADER_METHOD;
	const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
		pangolin_bridge_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
	const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
		pangolin_bridge_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
	const AVERAGE_BLOCK_INTERVAL: Duration = PangolinChain::AVERAGE_BLOCK_INTERVAL;
	const BRIDGE_CHAIN_ID: ChainId = pangolin_bridge_primitives::PANGOLIN_CHAIN_ID;
	type SigningParams = drml_primitives::SigningParams;
}

impl CliChain for PangolinChain {
	const RUNTIME_VERSION: RuntimeVersion = pangolin_runtime::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
}

declare_relay_headers!(
	Pangolin,
	Millau,
	PangolinChain,
	MillauChain,
	pangolin_bridge_relay_client_definition,
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
	PangolinChain,
	MillauChain,
	pangolin_bridge_relay_client_definition,
	relay_millau_client,
	PangolinChainConst,
	MillauChainConst,
	drml_primitives,
	bp_millau,
	pangolin_runtime,
	millau_runtime,
	SourceChainRuntimeMessagesCall,
	TargetChainRuntimeMessagesCall,
	pangolin_runtime_system_params,
	bp_millau,
	WithPangolinMessagesInstance,
	WithMillauMessagesInstance,
	MillauToPangolinConversionRate,
	INITIAL_MILLAU_TO_PANGOLIN_CONVERSION_RATE,
);
