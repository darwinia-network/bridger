use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};

use crate::bus::DarwiniaLinkedBus;
use crate::config::DarwiniaLinkedConfig;
use crate::route;
use crate::service::extrinsic::ExtrinsicService;

#[derive(Debug)]
pub struct DarwiniaLinked {
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for DarwiniaLinked {
    const NAME: &'static str = "linked-darwinia";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaLinked {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn route(
        &self,
        uri: String,
        param: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let ret = (uri, param);
        let value: serde_json::Value = serde_json::to_value(ret)?;
        Ok(value)
    }
}

impl BridgeTask<DarwiniaLinkedBus> for DarwiniaLinked {
    fn bus(&self) -> &DarwiniaLinkedBus {
        crate::bus::bus()
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl DarwiniaLinked {
    pub async fn new(config: DarwiniaLinkedConfig) -> anyhow::Result<Self> {
        config.store(DarwiniaLinked::NAME)?;
        let bus = crate::bus::bus();

        let services = vec![Self::spawn_service::<ExtrinsicService>(&bus)?];

        let carries = vec![];
        Ok(Self { services, carries })
    }
}
