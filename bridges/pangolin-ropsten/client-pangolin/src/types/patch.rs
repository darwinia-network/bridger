use codec::Encode;
use std::collections::HashMap;
use subxt::extrinsic::ChargeAssetTxPayment;
use subxt::DefaultExtraWithTxPayment;

use crate::config::PangolinSubxtConfig;
use crate::types::{
    darwinia_bridge_ethereum, darwinia_relay_primitives, ethereum_primitives, Balance,
};

/// Real realy affirmation types
pub type BetterRelayAffirmation = darwinia_relay_primitives::relayer_game::RelayAffirmation<
    darwinia_bridge_ethereum::EthereumRelayHeaderParcel,
    <PangolinSubxtConfig as subxt::Config>::AccountId,
    Balance,
    u64,
>;

/// Affirmations return data types
pub type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<BetterRelayAffirmation>>>;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra =
    DefaultExtraWithTxPayment<PangolinSubxtConfig, ChargeAssetTxPayment<PangolinSubxtConfig>>;

/// Ethereum receipt proof
pub struct EthereumReceiptProofThing {
    /// Ethereum header
    pub header: ethereum_primitives::header::Header,
    /// Receipt proof
    pub receipt_proof: ethereum_primitives::receipt::ReceiptProof,
    /// MMR proof
    pub mmr_proof: darwinia_bridge_ethereum::MMRProof,
}

/// Encode mmr root message
#[derive(Encode)]
pub struct _S<_1, _2, _3, _4>
where
    _1: Encode,
    _2: Encode,
    _3: Encode,
    _4: Encode,
{
    /// spec name
    pub _1: _1,
    /// op code, mmr root: 0x479fbdf9, next authorities: 0xb4bcf497
    pub _2: _2,
    /// block_number or term
    #[codec(compact)]
    pub _3: _3,
    /// mmr_root or next authorities
    pub _4: _4,
}
