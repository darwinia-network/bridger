use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask, BridgeTaskManage};

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

impl BridgeTask<TemplateTaskBus> for TemplateTask {}

impl BridgeTaskManage<TemplateTaskBus> for TemplateTask {
    fn bus(&self) -> &TemplateTaskBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl TemplateTask {
    pub fn new(config: TemplateTaskConfig) -> anyhow::Result<Self> {
        config.store(TemplateTask::NAME)?;
        let bus = TemplateTaskBus::default();

        let services = vec![Self::spawn_service::<SomeService>(&bus)?];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
