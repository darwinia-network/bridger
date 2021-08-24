use component_state::state::BridgeState;

mod v046;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    migrate_spec_version(state, current_version)
}

pub fn migrate_spec_version(state: &BridgeState, version: String) -> anyhow::Result<()> {
    match &version[..] {
        "0.4.6" => v046::migrate(state),
        _ => Ok(()),
    }
}
