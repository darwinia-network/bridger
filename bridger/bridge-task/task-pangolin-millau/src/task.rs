use serde::{Deserialize, Serialize};

use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};

use crate::bus::PangolinMillauBus;

#[derive(Debug)]
pub struct PangolinMillauTask {
    bus: PangolinMillauBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
}

impl BridgeTaskKeep for PangolinMillauTask {}

impl BridgeTask<PangolinMillauBus> for PangolinMillauTask {
    fn bus(&self) -> &PangolinMillauBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl PangolinMillauTask {
    pub async fn new(config: PangolinMillauConfig) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = PangolinMillauBus::default();

        let services = vec![];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}

// -- config --

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let _name = cell_name.as_ref();
        Ok(())
    }
}
