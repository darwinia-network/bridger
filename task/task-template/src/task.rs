use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use component_state::state::BridgeState;

use crate::bus::TemplateTaskBus;
use crate::config::TemplateTaskConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateTask {
    stack: TaskStack<TemplateTaskBus>,
}

impl BridgeSand for TemplateTask {
    const NAME: &'static str = "task-template";
}

impl BridgeTask<TemplateTaskBus> for TemplateTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(TemplateTaskConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<TemplateTaskBus> {
        &mut self.stack
    }
}

#[async_trait::async_trait]
impl BridgeTaskKeep for TemplateTask {
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

impl TemplateTask {
    pub fn new(config: TemplateTaskConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(TemplateTask::NAME)?;
        let bus = TemplateTaskBus::default();
        bus.store_resource::<BridgeState>(state);
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}
