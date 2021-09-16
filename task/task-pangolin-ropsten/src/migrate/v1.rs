use microkv::namespace::NamespaceMicroKV;

use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::task::PangolinRopstenTask;

pub fn migrate_v1(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
    auto_start_scan(&microkv)?;
    Ok(())
}

fn auto_start_scan(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    let tracker_pangolin = Tracker::new(microkv.clone(), "scan.pangolin");
    tracker_pangolin.start_running()?;

    let tracker_ropsten = Tracker::new(microkv.clone(), "scan.ropsten");
    tracker_ropsten.start_running()?;

    Ok(())
}
