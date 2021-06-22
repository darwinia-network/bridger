// pub struct Service {
//     bus: BridgeBus,
// }
//
// impl Service {
//     pub fn with(bus: BridgeBus) -> Self {
//         Self { bus }
//     }
// }
//
// impl Service {
//     pub fn spawn_service<S: lifeline::Service>(&self) -> anyhow::Result<&Self> {
//         S::spawn(&self.bus)?;
//         Ok(self)
//     }
// }
