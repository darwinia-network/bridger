use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::{BridgeSand, BridgeTask, BridgeTaskKeep, TaskTerminal};

use crate::bus::PangolinMillauBus;
use crate::config::PangolinMillauConfig;
use crate::message::{PangolinMillauMessageReceive, PangolinMillauMessageSend};
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;
use lifeline::{Bus, Receiver, Sender};
use support_s2s::types::BridgeName;

#[derive(Debug)]
pub struct PangolinMillauTask {
    bus: PangolinMillauBus,
    services: Vec<Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for PangolinMillauTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(&self.bus, uri, param).await
    }
}

impl BridgeTask<PangolinMillauBus> for PangolinMillauTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(PangolinMillauConfig::template())?)
    }
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

        let mut sender = bus.tx::<PangolinMillauMessageSend>()?;
        let mut receiver = bus.rx::<PangolinMillauMessageReceive>()?;
        sender
            .send(PangolinMillauMessageSend::InitBridge(
                BridgeName::MillauToPangolin,
            ))
            .await?;
        while let Some(recv) = receiver.recv().await {
            match recv {
                PangolinMillauMessageReceive::FinishedInitBridge => break,
            }
        }
        sender
            .send(PangolinMillauMessageSend::InitBridge(
                BridgeName::PangolinToMillau,
            ))
            .await?;
        while let Some(recv) = receiver.recv().await {
            match recv {
                PangolinMillauMessageReceive::FinishedInitBridge => break,
            }
        }
        sender
            .send(PangolinMillauMessageSend::Relay(
                BridgeName::PangolinToMillau,
            ))
            .await?;

        let carries = vec![];
        Ok(Self {
            bus,
            services,
            carries,
        })
    }
}
