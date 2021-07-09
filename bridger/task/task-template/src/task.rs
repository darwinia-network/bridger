use lifeline::dyn_bus::DynBus;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};
use component_state::state::BridgeState;

use crate::bus::TemplateTaskBus;
use crate::config::TemplateTaskConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateTask {
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for TemplateTask {
    const NAME: &'static str = "task-template";
}

impl BridgeTask<TemplateTaskBus> for TemplateTask {
    fn bus(&self) -> &TemplateTaskBus {
        crate::bus::bus()
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
    async fn route(
        &self,
        uri: String,
        param: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        todo!()
    }
}

impl TemplateTask {
    pub fn new(config: TemplateTaskConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(TemplateTask::NAME)?;
        let bus = crate::bus::bus();
        bus.store_resource::<BridgeState>(state);

        let services = vec![Self::spawn_service::<SomeService>(&bus)?];

        let carries = vec![];
        Ok(Self { services, carries })
    }
}
