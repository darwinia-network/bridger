use bridge_traits::bridge::task::BridgeSand;
use linked_darwinia::task::DarwiniaLinked;
use task_darwinia_ethereum::task::DarwiniaEthereumTask;
use task_pangolin_millau::task::PangolinMillauTask;

pub fn init() -> anyhow::Result<()> {
    init_log();
    init_keep()?;
    Ok(())
}

fn init_log() {
    std::env::set_var(
        "RUST_LOG",
        r#"
        serde=info,
        lifeline=debug,
        darwinia_bridge=debug,
        task-darwinia-ethereum=trace,
		support_s2s=debug,
		bridge=info,
        evm_log_tracker=info,
        "#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

fn init_keep() -> anyhow::Result<()> {
    support_keep::task::add_available_tasks(vec![
        DarwiniaLinked::NAME,
        DarwiniaEthereumTask::NAME,
        PangolinMillauTask::NAME,
    ])
}
