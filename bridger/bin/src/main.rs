use bridge_service::service::relay::RelayService;

fn main() -> anyhow::Result<()> {
    // let bus = BridgeBus::default();
    // let service = Service::with(bus);
    // service.spawn_service::<RelayService>()?;
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
