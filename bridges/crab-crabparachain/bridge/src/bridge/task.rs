use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::header::{
    CrabToParachainHeaderRelayService, KusamaToCrabHeaderRelayService, ParaHeadRelayService,
};
use crate::service::message::crab_to_crabparachain::CrabToCrabParachainMessageRelayService;
use crate::service::message::crabparachain_to_crab::CrabParachainToCrabMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "task-crab-crabparachain"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<CrabToParachainHeaderRelayService>()?;
        stack.spawn_service::<KusamaToCrabHeaderRelayService>()?;
        stack.spawn_service::<ParaHeadRelayService>()?;
        stack.spawn_service::<CrabToCrabParachainMessageRelayService>()?;
        stack.spawn_service::<CrabParachainToCrabMessageRelayService>()?;
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
