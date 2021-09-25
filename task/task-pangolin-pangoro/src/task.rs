use lifeline::{Bus, Sender};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};

use crate::bus::PangolinPangoroBus;
use crate::config::{PangolinPangoroConfig, RelayConfig};
use crate::message::PangolinPangoroMessageSend;
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct PangolinPangoroTask {
    stack: TaskStack<PangolinPangoroBus>,
}

impl BridgeSand for PangolinPangoroTask {
    const NAME: &'static str = "task-pangolin-pangoro";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for PangolinPangoroTask {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal> {
        crate::route::dispatch_route(self.stack.bus(), uri, param).await
    }
}

impl BridgeTask<PangolinPangoroBus> for PangolinPangoroTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(PangolinPangoroConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<PangolinPangoroBus> {
        &mut self.stack
    }
}

impl PangolinPangoroTask {
    pub async fn new(config: PangolinPangoroConfig) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;

        let bus = PangolinPangoroBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<InitBridgeService>()?;
        stack.spawn_service::<RelayService>()?;

        let mut sender = stack.bus().tx::<PangolinPangoroMessageSend>()?;
        let relay_config: RelayConfig = Config::restore(Self::NAME)?;
        if relay_config.auto_start {
            sender.send(PangolinPangoroMessageSend::Relay).await?;
        }

        Ok(Self { stack })
    }
}
