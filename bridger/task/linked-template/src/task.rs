use bridge_component::state::BridgeState;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};

use crate::bus::TemplateLinkedBus;
use crate::config::TemplateLinkedConfig;
use crate::service::some::SomeService;
use lifeline::dyn_bus::DynBus;

#[derive(Debug)]
pub struct TemplateLinked {
    bus: TemplateLinkedBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for TemplateLinked {
    const NAME: &'static str = "linked-template";
}

impl BridgeTaskKeep for TemplateLinked {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BridgeTask<TemplateLinkedBus> for TemplateLinked {
    fn bus(&self) -> &TemplateLinkedBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl TemplateLinked {
    pub fn new(config: TemplateLinkedConfig, state: BridgeState) -> anyhow::Result<Self> {
        config.store(TemplateLinked::NAME)?;
        let bus = TemplateLinkedBus::default();
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
