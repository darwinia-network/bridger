use lifeline::dyn_bus::DynBus;
use serde::{Deserialize, Serialize};

use bridge_shared::shared::SharedChannel;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::{BridgeSand, BridgeTask};

use crate::bus::PangolinMillauBus;
use crate::config::PangolinMillauConfig;
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct PangolinMillauTask {
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
}

impl BridgeTask for PangolinMillauTask {}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
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

impl PangolinMillauTask {
    fn spawn_service<
        S: lifeline::Service<Bus = PangolinMillauBus, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        bus: &PangolinMillauBus,
    ) -> anyhow::Result<Box<dyn BridgeService + Send + Sync>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}
