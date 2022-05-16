use std::collections::HashMap;

#[cfg(feature = "ethlike-v1")]
use codec::Encode;
use crate::subxt_runtime::api::runtime_types::{darwinia_bridge_ethereum, darwinia_relay_primitives};
use subxt::DefaultExtra;

use crate::config::PangolinSubxtConfig;
use crate::types::Balance;

/// Real relay affirmation types
pub type BetterRelayAffirmation = darwinia_relay_primitives::relayer_game::RelayAffirmation<
    darwinia_bridge_ethereum::EthereumRelayHeaderParcel,
    <PangolinSubxtConfig as subxt::Config>::AccountId,
    Balance,
    u64,
>;

/// Affirmations return data types
pub type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<BetterRelayAffirmation>>>;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<PangolinSubxtConfig>;

/// Encode mmr root message
#[cfg(feature = "ethlike-v1")]
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
