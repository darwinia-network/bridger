use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};

use crate::bus::DarwiniaLinkedBus;
use crate::config::DarwiniaLinkedConfig;
use crate::service::extrinsic::ExtrinsicService;

#[derive(Debug)]
pub struct DarwiniaLinked {
    bus: DarwiniaLinkedBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for DarwiniaLinked {
    const NAME: &'static str = "linked-darwinia";
}

impl BridgeTaskKeep for DarwiniaLinked {}

impl BridgeTask<DarwiniaLinkedBus> for DarwiniaLinked {
    fn bus(&self) -> &DarwiniaLinkedBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl DarwiniaLinked {
    pub async fn new(config: DarwiniaLinkedConfig) -> anyhow::Result<Self> {
        config.store(DarwiniaLinked::NAME)?;
        let bus = DarwiniaLinkedBus::default();

        let services = vec![Self::spawn_service::<ExtrinsicService>(&bus)?];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
