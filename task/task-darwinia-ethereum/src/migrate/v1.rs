use microkv::namespace::NamespaceMicroKV;

use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::task::DarwiniaEthereumTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    auto_start_scan(&microkv)?;
    Ok(())
}

fn auto_start_scan(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if microkv.get("scan.darwinia.next")?.is_some() {
        let tracker_pangolin = Tracker::new(microkv.clone(), "scan.darwinia");
        tracker_pangolin.start_running()?;
    }

    if microkv.get("scan.ethereum.next")?.is_some() {
        let tracker_ropsten = Tracker::new(microkv.clone(), "scan.ethereum");
        tracker_ropsten.start_running()?;
    }

    Ok(())
}
