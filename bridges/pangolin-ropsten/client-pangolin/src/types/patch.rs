use std::collections::HashMap;

use crate::config::PangolinSubxtConfig;
use crate::types;
use crate::types::Balance;

/// Real realy affirmation types
pub type BetterRelayAffirmation = types::darwinia_relay_primitives::relayer_game::RelayAffirmation<
    types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel,
    PangolinSubxtConfig::AccountId,
    Balance,
    types::darwinia_relay_primitives::relayer_game::RelayAffirmationId<u64>,
>;

/// Affirmations return data types
pub type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<BetterRelayAffirmation>>>;
