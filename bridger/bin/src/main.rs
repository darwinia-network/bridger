#[macro_use]
extern crate log;

use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
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
		darwinia_bridge=debug,
		service_darwinia_ethereum=debug,
        task-darwinia-ethereum=debug,
		"#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

fn config() -> DarwiniaEthereumConfig {
    let ethereum_endpoint = format!("https://mainnet.infura.io/v3/{}", env!("ETHEREUM_KEY"));
    let mut microkv_path = std::env::temp_dir();
    microkv_path.push("microkv/bridger");

    DarwiniaEthereumConfig {
        bee: BeeConfig {
            endpoint: "wss://rpc.darwinia.network".to_string(),
            strict: false,
        },
        web3: Web3Config {
            endpoint: ethereum_endpoint.clone(),
        },
        ethereum_rpc: EthereumRpcConfig {
            rpc: vec![ethereum_endpoint.clone()],
            atom: 0,
        },
        shadow: ShadowConfig {
            endpoint: "https://shadow.darwinia.network".to_string(),
        },
        microkv: MicrokvConfig {
            base_path: microkv_path,
            db_name: Some(DarwiniaEthereumTask::NAME.to_string()),
        },
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
    let mut task = DarwiniaEthereumTask::with(self::config())?;

    task.spawn_service::<LikeDarwiniaWithLikeEthereumRelayService<DarwiniaEthereumTask>>()?;
    task.spawn_service::<LikeDarwiniaWithLikeEthereumEthereumScanService<DarwiniaEthereumTask>>()?;
    task.start().await?;

    let mut times = 0;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        task.send_scan().await?;
        times += 1;
        if times == u64::MAX {
            drop(task);
            debug!("The task is stopped");
            break;
        }
    }
    loop {
        debug!("No task run");
        tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        break;
    }
    Ok(())
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
