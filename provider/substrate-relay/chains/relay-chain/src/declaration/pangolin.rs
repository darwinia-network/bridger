pub use pangolin_runtime;

use bp_header_chain::justification::GrandpaJustification;
use codec::Encode;
use millau_runtime::{BridgeGrandpaPangolinCall, WithPangolinGrandpaInstance};
use pangolin_runtime_params::s2s as s2s_params;
use relay_millau_client::Millau as MillauRelayChain;
use relay_pangolin_client::{
	PangolinRelayChain, SigningParams as PangolinSigningParams, SyncHeader as PangolinSyncHeader,
};
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};
use sp_version::RuntimeVersion;

use crate::types::s2s::finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate};
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
	PangolinSigningParams,
	PangolinChainConst,
	drml_primitives,
	bp_millau,
	millau_runtime,
	BridgeGrandpaPangolinCall,
	WithPangolinGrandpaInstance,
	PangolinSyncHeader,
);
