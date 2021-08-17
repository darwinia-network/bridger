use microkv::namespace::NamespaceMicroKV;

use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::task::PangolinRopstenTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
    migrate_scan_pangolin(&microkv)?;
    Ok(())
}

fn migrate_scan_pangolin(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    let block_pangolin: Option<u64> = microkv.get_as("last-tracked-pangolin-block")?;
    if let Some(block) = block_pangolin {
        let tracker = Tracker::new(microkv.clone(), "scan.pangolin");
        tracker.queue(vec![block as usize])?;
        microkv.delete("last-tracked-pangolin-block")?;
    }
    Ok(())
}
