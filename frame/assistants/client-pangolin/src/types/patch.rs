use std::collections::HashMap;

use subxt::DefaultExtra;

use crate::config::PangolinSubxtConfig;
use crate::subxt_runtime::api::runtime_types::{darwinia_bridge_ethereum, dp_relayer_game};
use crate::types::Balance;

/// Real relay affirmation types
pub type BetterRelayAffirmation = dp_relayer_game::RelayAffirmation<
    darwinia_bridge_ethereum::EthereumRelayHeaderParcel,
    <PangolinSubxtConfig as subxt::Config>::AccountId,
    Balance,
    u64,
>;

/// Affirmations return data types
pub type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<BetterRelayAffirmation>>>;

/// Node runtime signed extra
pub type NodeRuntimeSignedExtra = DefaultExtra<PangolinSubxtConfig>;
