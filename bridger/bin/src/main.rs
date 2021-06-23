use bridge_service::service::relay::SubstrateToEthereumRelayService;
use bridge_task::task::darwinia_ethereum::{DarwiniaEthereumConfig, DarwiniaEthereumTask};

fn config() -> DarwiniaEthereumConfig {
    DarwiniaEthereumConfig {
        bee: Default::default(),
        web3: Default::default(),
        ethereum_rpc: Default::default(),
        shadow: Default::default(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let task = DarwiniaEthereumTask::with(self::config())?;
    task.spawn_service::<SubstrateToEthereumRelayService<DarwiniaEthereumTask>>()?;

    drop(task);
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
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
