use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use microkv::namespace::NamespaceMicroKV;

use crate::task::PangolinRopstenTask;

pub fn migrate(state: &BridgeState) -> anyhow::Result<()> {
    let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
    migrate_tracker_ropsten(&microkv)?;
    migrate_tracker_pangolin(&microkv)?;
    migrate_affirm(&microkv)?;
    Ok(())
}

fn migrate_tracker_ropsten(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("scan.ropsten.finish")? {
        if value.is_number() {
            let last_block = value.as_u64().unwrap();
            microkv.put("scan.ropsten.redeem.current", &last_block)?;
            microkv.put("scan.ropsten.check.current", &last_block)?;
            microkv.put("scan.ropsten.affirm.current", &last_block)?;
        }
    }
    if let Some(value) = microkv.get("scan.ropsten.running")? {
        let mut is_running = false;
        if value.is_boolean() {
            is_running = value.as_bool().unwrap_or(false);
        }
        if value.is_string() {
            is_running = value.as_str().map_or(false, |v| v == "true");
        }
        if is_running {
            microkv.put("scan.ropsten.redeem.running", &true)?;
            microkv.put("scan.ropsten.check.running", &true)?;
            microkv.put("scan.ropsten.affirm.running", &true)?;
        }
    }
    for key in &[
        "scan.ropsten.running",
        "scan.ropsten.finish",
        "scan.ropsten.current",
        "scan.ropsten.next",
        "scan.ropsten.skipped",
        "scan.ropsten.fast_mode",
    ] {
        microkv.delete(key)?;
    }
    Ok(())
}

fn migrate_tracker_pangolin(microkv: &NamespaceMicroKV) -> anyhow::Result<()> {
    if let Some(value) = microkv.get("scan.pangolin.finish")? {
        if value.is_number() {
            microkv.put("scan.pangolin.current", &value.as_u64().unwrap_or(0))?;
        }
    }
    for key in &[
        "scan.pangolin.finish",
        "scan.pangolin.next",
        "scan.pangolin.skipped",
        "scan.pangolin.fast_mode",
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
        microkv.delete("target")?;
    }
    if let Some(value) = microkv.get("relayed")? {
        if value.is_number() {
            microkv.put("affirm.relayed", &value.as_u64().unwrap_or(0))?;
        }
        microkv.delete("relayed")?;
    }
    Ok(())
}
