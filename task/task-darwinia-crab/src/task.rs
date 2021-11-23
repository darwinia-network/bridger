use lifeline::{Bus, Sender};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::task::{
    BridgeSand, BridgeTask, BridgeTaskKeep, TaskStack, TaskTerminal,
};

use crate::bus::DarwiniaCrabBus;
use crate::config::{DarwiniaCrabConfig, RelayConfig};
use crate::message::PangolinPangoroMessageSend;
use crate::service::fee::UpdateFeeService;
use crate::service::init::InitBridgeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct DarwiniaCrabTask {
    stack: TaskStack<DarwiniaCrabBus>,
}

impl BridgeSand for DarwiniaCrabTask {
    const NAME: &'static str = "task-darwinia-crab";
}

#[async_trait::async_trait]
impl BridgeTaskKeep for DarwiniaCrabTask {
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

impl BridgeTask<DarwiniaCrabBus> for DarwiniaCrabTask {
    fn config_template() -> anyhow::Result<serde_json::Value> {
        Ok(serde_json::to_value(DarwiniaCrabConfig::template())?)
    }

    fn stack(&mut self) -> &mut TaskStack<DarwiniaCrabBus> {
        &mut self.stack
    }
}

impl DarwiniaCrabTask {
    pub async fn new(config: DarwiniaCrabConfig) -> anyhow::Result<Self> {
        config.store(Self::NAME)?;

        let bus = DarwiniaCrabBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<UpdateFeeService>()?;
        stack.spawn_service::<InitBridgeService>()?;
        stack.spawn_service::<RelayService>()?;

        let mut sender = stack.bus().tx::<PangolinPangoroMessageSend>()?;
        let relay_config: RelayConfig = Config::restore_unwrap(Self::NAME)?;
        if relay_config.auto_start {
            sender.send(PangolinPangoroMessageSend::Relay).await?;
        }

        Ok(Self { stack })
    }
}
