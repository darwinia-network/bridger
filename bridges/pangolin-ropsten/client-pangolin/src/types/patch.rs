use std::collections::HashMap;
use subxt::extrinsic::ChargeAssetTxPayment;
use subxt::DefaultExtraWithTxPayment;

use crate::config::PangolinSubxtConfig;
use crate::types;
use crate::types::{
    darwinia_bridge_ethereum, darwinia_relay_primitives, ethereum_primitives, Balance,
};

/// Real realy affirmation types
pub type BetterRelayAffirmation = darwinia_relay_primitives::relayer_game::RelayAffirmation<
    darwinia_bridge_ethereum::EthereumRelayHeaderParcel,
    PangolinSubxtConfig::AccountId,
    Balance,
    darwinia_relay_primitives::relayer_game::RelayAffirmationId<u64>,
>;

/// Affirmations return data types
pub type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<BetterRelayAffirmation>>>;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra =
    DefaultExtraWithTxPayment<PangolinSubxtConfig, ChargeAssetTxPayment<PangolinSubxtConfig>>;

/// Ethereum receipt proof
#[derive(Clone, Debug)]
pub struct EthereumReceiptProofThing {
    /// Ethereum header
    pub header: ethereum_primitives::header::Header,
    /// Receipt proof
    pub receipt_proof: ethereum_primitives::receipt::ReceiptProof,
    /// MMR proof
    pub mmr_proof: darwinia_bridge_ethereum::MMRProof,
}
