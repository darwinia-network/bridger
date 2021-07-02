use std::{ops::RangeInclusive, time::Duration};

use millau_runtime::{
    pangolin_messages::{
        PangolinToMillauConversionRate, INITIAL_PANGOLIN_TO_MILLAU_CONVERSION_RATE,
    },
    BridgeMessagesCall as SourceChainRuntimeMessagesCall,
    WithPangolinMessages as WithPangolinMessagesInstance,
};
use pangolin_runtime::{
    BridgeGrandpaCall as BridgeGrandpaMillauCall,
    BridgeMessagesCall as TargetChainRuntimeMessagesCall,
    WithMillauGrandpa as WithMillauGrandpaInstance,
    WithMillauMessages as WithMillauMessagesInstance,
};

use bp_messages::MessageNonce;
use bp_runtime::ChainId;
use chain_millau::MillauChain;
use codec::Encode;
use frame_support::dispatch::GetDispatchInfo;
use messages_relay::message_lane::MessageLane;
use pangolin_bridge_relay_client_definition::PangolinChain;
use relay_millau_client::Millau as MillauChain;
use relay_substrate_client::{
    metrics::{FloatStorageValueMetric, StorageProofOverheadMetric},
    Chain as RelaySubstrateClientChain, TransactionSignScheme,
};
use sp_version::RuntimeVersion;

pub struct MillauChainConst;

impl ChainConst for MillauChainConst {
    const OUTBOUND_LANE_MESSAGES_DISPATCH_WEIGHT_METHOD: &'static str =
        millau_primitives::TO_MILLAU_MESSAGES_DISPATCH_WEIGHT_METHOD;
    const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
        millau_primitives::TO_MILLAU_LATEST_GENERATED_NONCE_METHOD;
    const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
        millau_primitives::TO_MILLAU_LATEST_RECEIVED_NONCE_METHOD;
    const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
        millau_primitives::FROM_MILLAU_LATEST_RECEIVED_NONCE_METHOD;
    const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
        millau_primitives::FROM_MILLAU_LATEST_CONFIRMED_NONCE_METHOD;
    const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
        millau_primitives::FROM_MILLAU_UNREWARDED_RELAYERS_STATE;
    const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
        millau_primitives::BEST_FINALIZED_MILLAU_HEADER_METHOD;
    const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
        millau_primitives::BEST_FINALIZED_MILLAU_HEADER_METHOD;
    const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce =
        millau_primitives::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE;
    const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce =
        millau_primitives::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
    const AVERAGE_BLOCK_INTERVAL: Duration = MillauChain::AVERAGE_BLOCK_INTERVAL;
    const BRIDGE_CHAIN_ID: ChainId = bp_runtime::MILLAU_CHAIN_ID;
    type SigningParams = relay_millau_client::SigningParams;
}

impl CliChain for MillauChain {
    const RUNTIME_VERSION: RuntimeVersion = millau_runtime::VERSION;

    type KeyPair = sp_core::sr25519::Pair;
}

external_s2s::declare_relay_headers!(
    Millau,
    Pangolin,
    MillauChain,
    PangolinChain,
    relay_millau_client,
    MillauChainConst,
    millau_primitives,
    drml_primitives,
    pangolin_runtime,
    BridgeGrandpaMillauCall,
    WithMillauGrandpaInstance,
);

external_s2s::declare_relay_messages!(
    Millau,
    Pangolin,
    MillauChain,
    PangolinChain,
    relay_millau_client,
    pangolin_bridge_relay_client_definition,
    MillauChainConst,
    PangolinChainConst,
    millau_primitives,
    drml_primitives,
    millau_runtime,
    pangolin_runtime,
    SourceChainRuntimeMessagesCall,
    TargetChainRuntimeMessagesCall,
    millau_primitives,
    pangolin_runtime_system_params,
    WithMillauMessagesInstance,
    WithPangolinMessagesInstance,
    PangolinToMillauConversionRate,
    INITIAL_PANGOLIN_TO_MILLAU_CONVERSION_RATE,
);
