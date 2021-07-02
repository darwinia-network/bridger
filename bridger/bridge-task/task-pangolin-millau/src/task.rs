use serde::{Deserialize, Serialize};

use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep};

use crate::bus::PangolinMillauBus;
use crate::config::PangolinMillauConfig;
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct PangolinMillauTask {
    bus: PangolinMillauBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
}

impl BridgeTaskKeep for PangolinMillauTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BridgeTask<PangolinMillauBus> for PangolinMillauTask {
    fn bus(&self) -> &PangolinMillauBus {
        &self.bus
    }

    fn keep_carry(&mut self, other_bus: lifeline::Lifeline) {
        self.carries.push(other_bus);
    }
}

impl PangolinMillauTask {
    pub async fn new(config: PangolinMillauConfig, channel: SharedChannel) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = PangolinMillauBus::default();
        bus.store_resource::<SharedChannel>(channel);

        let services = vec![
            Self::spawn_service::<InitBridgeService>(&bus)?,
            Self::spawn_service::<RelayService>(&bus)?,
        ];

        Ok(Self { services })
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
