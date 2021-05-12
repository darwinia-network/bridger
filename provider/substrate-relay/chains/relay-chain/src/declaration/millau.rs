pub use millau_runtime;

use bp_header_chain::justification::GrandpaJustification;
use bp_millau;
use codec::Encode;
use pangolin_runtime::bridge::s2s::{BridgeGrandpaMillauCall, WithMillauGrandpaInstance};
use relay_millau_client::{
	Millau as MillauRelayChain, SigningParams as MillauSigningParams, SyncHeader as MillauSyncHeader,
};
use relay_pangolin_client::PangolinRelayChain;
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};
use sp_version::RuntimeVersion;

use crate::types::s2s::finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate};
use crate::*;

pub struct MillauChainConst;

impl ChainConst for MillauChainConst {
	const OUTBOUND_LANE_MESSAGES_DISPATCH_WEIGHT_METHOD: &'static str =
		bp_millau::TO_MILLAU_MESSAGES_DISPATCH_WEIGHT_METHOD;
	const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
		bp_millau::TO_MILLAU_LATEST_GENERATED_NONCE_METHOD;
	const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str = bp_millau::TO_MILLAU_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str = bp_millau::FROM_MILLAU_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
		bp_millau::FROM_MILLAU_LATEST_CONFIRMED_NONCE_METHOD;
	const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str = bp_millau::FROM_MILLAU_UNREWARDED_RELAYERS_STATE;
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = bp_millau::BEST_FINALIZED_MILLAU_HEADER_METHOD;
	const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str = bp_millau::BEST_FINALIZED_MILLAU_HEADER_METHOD;
}

declare_cli_chain!(MillauRelayChain, millau_runtime);

declare_relay_chain!(Millau, {
	const CHAIN_NAME: &'static str = "millau";
	const RUNTIME_VERSION: RuntimeVersion = millau_runtime::VERSION;
	type Runtime = millau_runtime::Runtime;
	type HeaderId = relay_millau_client::HeaderId;
	type Chain = relay_millau_client::Millau;
	type SigningParams = relay_millau_client::SigningParams;
	type SyncHeader = relay_millau_client::SyncHeader;
});

declare_relay_headers!(
	Millau,
	Pangolin,
	MillauRelayChain,
	PangolinRelayChain,
	MillauSigningParams,
	MillauChainConst,
	bp_millau,
	drml_primitives,
	pangolin_runtime,
	BridgeGrandpaMillauCall,
	WithMillauGrandpaInstance,
	MillauSyncHeader,
);
