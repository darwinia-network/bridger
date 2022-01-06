use std::time::Duration;

use bp_messages::MessageNonce;
use bp_runtime::ChainId;
use frame_support::weights::Weight;

/// Declare chain const type
pub trait ChainConst {
    /// Name of the runtime method that returns dispatch weight of outbound messages at the source chain.
    const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str;
    /// Name of the runtime method that returns latest generated nonce at the source chain.
    const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str;
    /// Name of the runtime method that returns latest received (confirmed) nonce at the the source chain.
    const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str;

    /// Name of the runtime method that returns latest received nonce at the target chain.
    const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str;
    /// Name of the runtime method that returns latest confirmed (reward-paid) nonce at the target chain.
    const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str;
    /// Numeber of the runtime method that returns state of "unrewarded relayers" set at the target chain.
    const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str;

    /// Name of the runtime method that returns id of best finalized source header at target chain.
    const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str;
    /// Name of the runtime method that returns id of best finalized target header at source chain.
    const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str;

    /// Maximal number of unrewarded relayer entries at inbound lane.
    const MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE: MessageNonce;
    /// Maximal number of unconfirmed messages at inbound lane.
    const MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE: MessageNonce;

    /// AVERAGE_BLOCK_INTERVAL
    const AVERAGE_BLOCK_INTERVAL: Duration;
    /// Bridge chain id
    const BRIDGE_CHAIN_ID: ChainId;

    /// Name of the messages pallet as it is declared in the `construct_runtime!()` at source chain.
    const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str;
    /// Name of the messages pallet as it is declared in the `construct_runtime!()` at target chain.
    const MESSAGE_PALLET_NAME_AT_TARGET: &'static str;
    /// Extra weight of the delivery transaction at the target chain, that is paid to cover
    /// dispatch fee payment.
    ///
    /// If dispatch fee is paid at the source chain, then this weight is refunded by the
    /// delivery transaction.
    const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight;

    /// chain signing params
    type SigningParams;
}

/// Bridge-supported network definition.
///
/// Used to abstract away CLI commands.
pub trait CliChain: relay_substrate_client::Chain {
    /// Chain's current version of the runtime.
    const RUNTIME_VERSION: sp_version::RuntimeVersion;

    /// Crypto keypair type used to send messages.
    ///
    /// In case of chains supporting multiple cryptos, pick one used by the CLI.
    type KeyPair: sp_core::crypto::Pair;
}
