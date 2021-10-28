use bridge_traits::bridge::task::BridgeSand;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_pangoro::task::PangolinPangoroTask;
use task_pangolin_ropsten::task::PangolinRopstenTask;

pub fn init() -> anyhow::Result<()> {
    init_log();
    init_keep()?;
    Ok(())
}

fn init_log() {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var(
            "RUST_LOG",
            r#"
        serde=info,
        lifeline=debug,
        darwinia_bridge=debug,
        bridge=info,
        support_tracker_evm_log=info,
        task-darwinia-ethereum=trace,
        task-pangolin-ropsten=trace,
        task-pangolin-pangoro=trace,
        "#,
        );
    }
    if let Err(_) = std::env::var("RUST_BACKTRACE") {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();
}

fn init_keep() -> anyhow::Result<()> {
    support_keep::task::add_available_tasks(vec![
        DarwiniaLinked::NAME,
        DarwiniaEthereumTask::NAME,
        PangolinPangoroTask::NAME,
        PangolinRopstenTask::NAME,
    ])
}
