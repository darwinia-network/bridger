use bridge_service::service::relay::RelayService;
use bridge_standard::bridge::task::BridgeTask;
use bridge_task::bus::DarwiniaEthereumBus;
use bridge_task::TaskDarwiniaEthereum;
use chain_darwinia::chain::DarwiniaChain;

fn main() -> anyhow::Result<()> {
    let bus_darwinia_ethereum = DarwiniaEthereumBus::default();

    let task_darwinia_ethereum = TaskDarwiniaEthereum {};

    drop(bus_darwinia_ethereum);
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
