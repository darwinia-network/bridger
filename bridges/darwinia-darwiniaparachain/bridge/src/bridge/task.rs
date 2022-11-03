use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::header::{
    DarwiniaToParachainHeaderRelayService, ParaHeadRelayService,
    PolkadotToDarwiniaHeaderRelayService,
};
use crate::service::message::darwinia_to_darwiniaparachain::DarwiniaToDarwiniaParachainMessageRelayService;
use crate::service::message::darwiniaparachain_to_darwinia::DarwiniaParachainToDarwiniaMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "task-darwinia-darwiniaparachain"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<DarwiniaToParachainHeaderRelayService>()?;
        stack.spawn_service::<PolkadotToDarwiniaHeaderRelayService>()?;
        stack.spawn_service::<ParaHeadRelayService>()?;
        stack.spawn_service::<DarwiniaToDarwiniaParachainMessageRelayService>()?;
        stack.spawn_service::<DarwiniaParachainToDarwiniaMessageRelayService>()?;
        stack.spawn_service::<SubscribeService>()?;
        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
