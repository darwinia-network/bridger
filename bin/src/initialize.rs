use std::env;

use bridge_traits::bridge::task::BridgeSand;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_crab::task::DarwiniaCrabTask;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_pangoro::task::PangolinPangoroTask;
use task_pangolin_ropsten::task::PangolinRopstenTask;

pub fn init() -> anyhow::Result<()> {
    init_log();
    init_keep()?;
    Ok(())
}

fn init_log() {
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            [
                "serde=info",
                "lifeline=debug",
                "darwinia_bridge=debug",
                "bridge=info",
                "component_pangolin_subxt=trace",
                "subscan=trace",
                "support_tracker_evm_log=info",
                "task-darwinia-ethereum=trace",
                "task-pangolin-ropsten=trace",
                "task-pangolin-pangoro=trace",
                "task-darwinia-crab=trace",
                "jsonrpsee_ws_client=error",
                "component_pangoro_s2s=trace",
                "component_pangolin_s2s=trace",
                "component_darwinia_s2s=trace",
                "component_crab_s2s=trace",
            ]
            .join(","),
        );
    }
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    env_logger::init();
}

fn init_keep() -> anyhow::Result<()> {
    support_keep::task::add_available_tasks(vec![
        DarwiniaLinked::NAME,
        DarwiniaEthereumTask::NAME,
        PangolinPangoroTask::NAME,
        PangolinRopstenTask::NAME,
        DarwiniaCrabTask::NAME,
    ])
}
