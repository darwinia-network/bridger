use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::DarwiniaEthereumConfig;
use crate::service::starter::StarterService;

#[derive(Debug)]
pub struct DarwiniaEthereumTask {
    stack: TaskStack<DarwiniaEthereumBus>,
}

impl BridgeSand for DarwiniaEthereumTask {
    const NAME: &'static str = "task-darwinia-ethereum";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaEthereumTask {
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

impl BridgeTask<DarwiniaEthereumBus> for DarwiniaEthereumTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(DarwiniaEthereumConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<DarwiniaEthereumBus> {
        &mut self.stack
    }
}

impl DarwiniaEthereumTask {
    pub async fn new(config: DarwiniaEthereumConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = DarwiniaEthereumBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<StarterService>()?;

        Ok(Self { stack })
    }
}
