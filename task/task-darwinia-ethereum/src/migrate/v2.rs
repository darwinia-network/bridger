use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use microkv::namespace::NamespaceMicroKV;

use crate::task::DarwiniaEthereumTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
    migrate_tracker_ethereum(&microkv)?;
    migrate_tracker_darwinia(&microkv)?;
    migrate_affirm(&microkv)?;
    Ok(())
}

fn migrate_tracker_ethereum(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    for key in &[
        "scan.ethereum.running",
        "scan.ethereum.finish",
        "scan.ethereum.current",
        "scan.ethereum.next",
        "scan.ethereum.skipped",
        "scan.ethereum.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    microkv.put("scan.ethereum.redeem.running", &true)?;
    microkv.put("scan.ethereum.check.running", &true)?;
    microkv.put("scan.ethereum.affirm.running", &true)?;
    if let Some(value) = microkv.get("scan.ethereum.finish")? {
        if value.is_number() {
            let last_block = value.as_u64().unwrap();
            microkv.put("scan.ethereum.redeem.current", &last_block)?;
            microkv.put("scan.ethereum.check.current", &last_block)?;
            microkv.put("scan.ethereum.affirm.current", &last_block)?;
        }
    }
    Ok(())
}

fn migrate_tracker_darwinia(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("scan.darwinia.finish")? {
        if value.is_number() {
            microkv.put("scan.darwinia.current", &value.as_u64().unwrap_or(0))?;
        }
    }
    for key in &[
        "scan.darwinia.finish",
        "scan.darwinia.next",
        "scan.darwinia.skipped",
        "scan.darwinia.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_affirm(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("target")? {
        if value.is_number() {
            microkv.put("affirm.target", &value.as_u64().unwrap_or(0))?;
        }
    }
    if let Some(value) = microkv.get("relayed")? {
        if value.is_number() {
            microkv.put("affirm.relayed", &value.as_u64().unwrap_or(0))?;
        }
    }
    microkv.delete("target")?;
    microkv.delete("relayed")?;
    Ok(())
}
