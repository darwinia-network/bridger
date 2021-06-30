#[macro_use]
extern crate log;

use bridge_config::config::component::{
    BeeConfig, EthereumRpcConfig, MicrokvConfig, ShadowConfig, Web3Config,
};
use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_shared::shared::BridgeShared;
use bridge_shared::shared::{DarwiniaServiceConfig, SharedConfig};
use bridge_standard::bridge::task::BridgeSand;
use task_darwinia_ethereum::task::{DarwiniaEthereumConfig, DarwiniaEthereumTask};

fn init() {
    std::env::set_var(
        "RUST_LOG",
        r#"
        serde=info,
        lifeline=debug,
        darwinia_bridge=debug,
        bridge_shared=debug,
        shared-darwinia=debug,
        service_darwinia_ethereum=debug,
        task-darwinia-ethereum=debug,
        "#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

#[allow(clippy::redundant_clone)]
fn config_task() -> DarwiniaEthereumConfig {
    let ethereum_key = option_env!("ETHEREUM_KEY").unwrap_or("hello");
    let ethereum_endpoint = format!("https://mainnet.infura.io/v3/{}", ethereum_key);
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
            auto_commit: true,
        },
        service: SubstrateEthereumConfig {
            interval_ethereum: 120,
            interval_relay: 60,
            interval_redeem: 150,
            interval_guard: 30,
        },
    }
}

fn config_shared() -> SharedConfig {
    SharedConfig {
        service_darwinia: DarwiniaServiceConfig {
            bee: BeeConfig {
                endpoint: "wss://rpc.darwinia.network".to_string(),
                strict: false,
            },
        },
    }
}

#[allow(clippy::never_loop)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::init();

    // init bridge shared
    let shared = BridgeShared::new(self::config_shared())?;
    let channel = shared.channel();

    // darwinia ethereum bridge
    let task = DarwiniaEthereumTask::new(self::config_task(), channel.clone()).await?;

    let mut times = 0;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
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
