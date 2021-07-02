use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask, BridgeTaskManage};

use crate::bus::TemplateLinkedBus;
use crate::config::TemplateLinkedConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateLinked {
    bus: TemplateLinkedBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for TemplateLinked {
    const NAME: &'static str = "linked-template";
}

impl BridgeTaskManage for TemplateLinked {}

impl BridgeTask<TemplateLinkedBus> for TemplateLinked {
    fn bus(&self) -> &TemplateLinkedBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl TemplateLinked {
    pub fn new(config: TemplateLinkedConfig) -> anyhow::Result<Self> {
        config.store(TemplateLinked::NAME)?;
        let bus = TemplateLinkedBus::default();

        let services = vec![Self::spawn_service::<SomeService>(&bus)?];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
