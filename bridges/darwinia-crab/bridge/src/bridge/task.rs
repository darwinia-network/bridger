use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::feemarket::FeemarketService;
use crate::service::header::crab_to_darwinia::CrabToDarwiniaHeaderRelayService;
use crate::service::header::darwinia_to_crab::DarwiniaToCrabHeaderRelayService;
use crate::service::message::crab_to_darwinia::CrabToDarwiniaMessageRelayService;
use crate::service::message::darwinia_to_crab::DarwiniaToCrabMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "darwinia-crab"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SubscribeService>()?;
        stack.spawn_service::<CrabToDarwiniaHeaderRelayService>()?;
        stack.spawn_service::<DarwiniaToCrabHeaderRelayService>()?;
        stack.spawn_service::<CrabToDarwiniaMessageRelayService>()?;
        stack.spawn_service::<DarwiniaToCrabMessageRelayService>()?;
        stack.spawn_service::<FeemarketService>()?;

        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
