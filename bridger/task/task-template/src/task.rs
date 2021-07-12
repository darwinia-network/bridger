use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep, TaskTerminal};
use component_state::state::BridgeState;

use crate::bus::TemplateTaskBus;
use crate::config::TemplateTaskConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateTask {
    bus: TemplateTaskBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for TemplateTask {
    const NAME: &'static str = "task-template";
}

impl BridgeTask<TemplateTaskBus> for TemplateTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(TemplateTaskConfig::template())?)
    }
    fn bus(&self) -> &TemplateTaskBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

#[async_trait::async_trait]
impl BridgeTaskKeep for TemplateTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(&self.bus, uri, param).await
    }
}

impl TemplateTask {
    pub fn new(config: TemplateTaskConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(TemplateTask::NAME)?;
        let bus = TemplateTaskBus::default();
        bus.store_resource::<BridgeState>(state);

        let services = vec![Self::spawn_service::<SomeService>(&bus)?];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
