use bridge_traits::bridge::task::BridgeSand;
use bridge_traits::error::StandardError;
use component_state::state::BridgeState;

use crate::task::DarwiniaEthereumTask;

mod v0;
mod v1;

pub fn migrate(state: &BridgeState, version: usize) -> anyhow::Result<()> {
    let saved_version = current_version(state)?;
    // same version, no migrate
    if saved_version == version {
        return Ok(());
    }

    let steps: Vec<Box<dyn Fn(&BridgeState) -> anyhow::Result<()>>> =
        vec![Box::new(v0::migrate), Box::new(v1::migrate)];

    let max_version = steps.len() - 1;
    if version > max_version {
        return Err(StandardError::Migration(format!(
            "Support max version: {}, but want upgrade to {}.",
            max_version, version
        ))
        .into());
    }
    let from = if saved_version == 0 {
        0
    } else {
        saved_version + 1
    };
    let to = version + 1;
    for ix in from..to {
        let migration = steps.get(ix).unwrap();
        if let Err(e) = migration(state) {
            return Err(StandardError::Migration(format!(
                "Failed to migrate. step [{}]: {:?}",
                ix, e
            ))
            .into());
        }
    }
    flush_version(state, version)?;
    Ok(())
}

fn current_version(state: &BridgeState) -> anyhow::Result<usize> {
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    let version: Option<usize> = microkv.get_as(".version")?;
    Ok(version.unwrap_or(0))
}

fn flush_version(state: &BridgeState, version: usize) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    microkv.put(".version", &version)?;
    Ok(())
}
