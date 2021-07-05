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
    pub async fn new(config: PangolinMillauConfig) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;
        let bus = PangolinMillauBus::default();

        let services = vec![
            Self::spawn_service::<InitBridgeService>(&bus)?,
            Self::spawn_service::<RelayService>(&bus)?,
        ];

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
