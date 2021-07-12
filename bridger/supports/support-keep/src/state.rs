use once_cell::sync::OnceCell;

use bridge_traits::error::StandardError;
use component_state::state::BridgeState;

static BRIDGE_STATE: OnceCell<BridgeState> = OnceCell::new();

pub fn set_state(state: BridgeState) -> anyhow::Result<()> {
    BRIDGE_STATE
        .set(state)
        .map_err(|_e| StandardError::Api("Failed to keep bridge state".to_string()).into())
}

pub fn get_state() -> Option<BridgeState> {
    BRIDGE_STATE.get().cloned()
}
