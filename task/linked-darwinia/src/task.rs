use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};

use crate::bus::DarwiniaLinkedBus;
use crate::config::DarwiniaLinkedConfig;
use crate::service::extrinsic::ExtrinsicService;

#[derive(Debug)]
pub struct DarwiniaLinked {
    stack: TaskStack<DarwiniaLinkedBus>,
}

impl BridgeSand for DarwiniaLinked {
    const NAME: &'static str = "linked-darwinia";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaLinked {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(self.stack.bus(), uri, param).await
    }
}

impl BridgeTask<DarwiniaLinkedBus> for DarwiniaLinked {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(DarwiniaLinkedConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<DarwiniaLinkedBus> {
        &mut self.stack
    }
}

impl DarwiniaLinked {
    pub async fn new(config: DarwiniaLinkedConfig) -> anyhow::Result<Self> {
        config.store(DarwiniaLinked::NAME)?;
        let bus = DarwiniaLinkedBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<ExtrinsicService>()?;
        Ok(Self { stack })
    }
}
