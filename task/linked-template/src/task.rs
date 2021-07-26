use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use component_state::state::BridgeState;

use crate::bus::TemplateLinkedBus;
use crate::config::TemplateLinkedConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateLinked {
    stack: TaskStack<TemplateLinkedBus>,
}

impl BridgeSand for TemplateLinked {
    const NAME: &'static str = "linked-template";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for TemplateLinked {
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

impl BridgeTask<TemplateLinkedBus> for TemplateLinked {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(TemplateLinkedConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<TemplateLinkedBus> {
        &mut self.stack
    }
}

impl TemplateLinked {
    pub fn new(config: TemplateLinkedConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(TemplateLinked::NAME)?;
        let bus = TemplateLinkedBus::default();
        bus.store_resource::<BridgeState>(state);

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}
