use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use microkv::namespace::NamespaceMicroKV;

use crate::task::DarwiniaEthereumTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    migrate_tracker_ethereum(&microkv)?;
    migrate_tracker_darwinia(&microkv)?;
    Ok(())
}

fn migrate_tracker_ethereum(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    for key in vec![
        "scan.ethereum.running",
        "scan.ethereum.finish",
        "scan.ethereum.current",
        "scan.ethereum.next",
        "scan.ethereum.skipped",
        "scan.ethereum.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    // todo: check there, save scan.ethereum.finish to scan.ethereum.redeem.current
    microkv.put("scan.ethereum.redeem.running", &true);
    microkv.put("scan.ethereum.check.running", &true);
    Ok(())
}

fn migrate_tracker_darwinia(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("scan.darwinia.finish")? {
        if value.is_number() {
            microkv.put("scan.darwinia.current", &value.as_u64().unwrap_or(0));
        }
    }
    for key in vec![
        "scan.darwinia.finish",
        "scan.darwinia.next",
        "scan.darwinia.skipped",
        "scan.darwinia.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}
