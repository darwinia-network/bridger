use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_standard::bridge::task::BridgeTask;
use bridge_task::bus::DarwiniaEthereumBus;
use bridge_task::task::darwinia_ethereum::{DarwiniaEthereumConfig, DarwiniaEthereumTask};
use service_darwinia_ethereum::message::s2e::EthereumScanMessage;
use service_darwinia_ethereum::service::ethereum::LikeDarwiniaWithLikeEthereumEthereumScanService;
use service_darwinia_ethereum::service::relay::LikeDarwiniaWithLikeEthereumRelayService;

fn init() {
    std::env::set_var(
        "RUST_LOG",
        r#"
		serde=info,
		lifeline=debug,
		bridge=info,
		"#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

fn config() -> DarwiniaEthereumConfig {
    DarwiniaEthereumConfig {
        bee: Default::default(),
        web3: Default::default(),
        ethereum_rpc: Default::default(),
        shadow: Default::default(),
        service: SubstrateEthereumConfig {
            interval_ethereum: 120,
            interval_relay: 60,
            interval_redeem: 150,
            interval_guard: 30,
        },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::init();

    // darwinia ethereum bridge
    let task = DarwiniaEthereumTask::with(self::config())?;
    let _s0 =
        task.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService<DarwiniaEthereumTask>>()?;
    let _s1 = task
        .spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService<DarwiniaEthereumTask>>()?;
    task.start().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        task.send_scan().await?;
    }
}

/*
// fake code
Task::with_name("darwinia-to-ethereum")
  .source(Darwinia::with(config))
  .target(Ethereum::with(config))
  .service(Service::relay())
  .service(Service::extrinsic())
  .build()
  .spawn()
 */
