use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};

use crate::bus::DarwiniaLinkedBus;
use crate::config::DarwiniaLinkedConfig;
use crate::service::extrinsic::ExtrinsicService;

#[derive(Debug)]
pub struct DarwiniaLinked {
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
}

impl BridgeSand for DarwiniaLinked {
    const NAME: &'static str = "linked-darwinia";
}

impl BridgeTask for DarwiniaLinked {}

impl DarwiniaLinked {
    pub async fn new(config: DarwiniaLinkedConfig) -> anyhow::Result<Self> {
        config.store(DarwiniaLinked::NAME)?;
        let bus = DarwiniaLinkedBus::default();

        let services = vec![Self::spawn_service::<ExtrinsicService>(&bus)?];

        Ok(Self { services })
    }
}

impl DarwiniaLinked {
    fn spawn_service<
        S: lifeline::Service<Bus = DarwiniaLinkedBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        bus: &DarwiniaLinkedBus,
    ) -> anyhow::Result<Box<dyn BridgeService + Send + Sync>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}
