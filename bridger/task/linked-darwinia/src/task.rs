use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep, TaskRouter};

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

impl BridgeTaskKeep for DarwiniaLinked {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BridgeTask<DarwiniaLinkedBus> for DarwiniaLinked {
    fn bus(&self) -> &DarwiniaLinkedBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }

    fn register_route(&self, router: &mut TaskRouter) {
        router.register::<DarwiniaLinked>("test", Box::new(move |x| Box::pin(Self::test_route(x))));
    }
}

impl DarwiniaLinked {
    async fn test_route(arg: String) -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::Value::String("Hello task route".to_string()))
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
