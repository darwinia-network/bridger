use lifeline::{Bus, Sender};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};
use support_s2s::types::BridgeName;

use crate::bus::PangolinMillauBus;
use crate::config::{PangolinMillauConfig, RelayConfig};
use crate::message::PangolinMillauMessageSend;
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct PangolinMillauTask {
    bus: PangolinMillauBus,
    stack: TaskStack<PangolinMillauBus>,
}

impl BridgeSand for PangolinMillauTask {
    const NAME: &'static str = "task-pangolin-millau";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for PangolinMillauTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
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

    fn stack(&mut self) -> &mut TaskStack<PangolinMillauBus> {
        &mut self.stack
    }
}

impl PangolinMillauTask {
    pub async fn new(config: PangolinMillauConfig) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;

        let bus = PangolinMillauBus::default();

        let mut stack = TaskStack::new();
        stack.spawn_service::<InitBridgeService>(&bus)?;
        stack.spawn_service::<RelayService>(&bus)?;

        let mut sender = bus.tx::<PangolinMillauMessageSend>()?;
        let relay_config: RelayConfig = Config::restore(Self::NAME)?;
        if relay_config.auto_start {
            sender
                .send(PangolinMillauMessageSend::Relay(
                    BridgeName::PangolinToMillau,
                ))
                .await?;
        }

        Ok(Self { bus, stack })
    }
}
