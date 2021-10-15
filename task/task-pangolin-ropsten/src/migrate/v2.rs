use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use microkv::namespace::NamespaceMicroKV;

use crate::task::PangolinRopstenTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
    migrate_tracker_ropsten(&microkv)?;
    migrate_tracker_pangolin(&microkv)?;
    Ok(())
}

fn migrate_tracker_ropsten(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    for key in [
        "scan.ropsten.running",
        "scan.ropsten.finish",
        "scan.ropsten.current",
        "scan.ropsten.next",
        "scan.ropsten.skipped",
        "scan.ropsten.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    microkv.put("scan.ropsten.redeem.running", &true)?;
    microkv.put("scan.ropsten.check.running", &true)?;
    Ok(())
}

fn migrate_tracker_pangolin(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("scan.pangolin.finish")? {
        if value.is_number() {
            microkv.put("scan.pangolin.current", &value.as_u64().unwrap_or(0))?;
        }
    }
    for key in [
        "scan.pangolin.finish",
        "scan.pangolin.next",
        "scan.pangolin.skipped",
        "scan.pangolin.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}
