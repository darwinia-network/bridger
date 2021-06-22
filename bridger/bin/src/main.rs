use bridge_service::service::relay::RelayService;
use bridge_task::TaskDarwiniaEthereum;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let task = TaskDarwiniaEthereum::new();
    task.spawn_service::<RelayService<TaskDarwiniaEthereum>>()?;

    drop(task);
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
