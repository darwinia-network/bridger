use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::error::{BinS2SError, BinS2SResult};
use crate::service::feemarket::FeemarketService;
#[cfg(feature = "solo-with-solo")]
use crate::service::solo_with_solo::{
    SourceToTargetHeaderRelayService, SourceToTargetMessageRelayService,
    TargetToSourceHeaderRelayService, TargetToSourceMessageRelayService,
};
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
    #[cfg(feature = "solo-with-solo")]
    fn spawn_relay_solo_with_solo(stack: &mut TaskStack<BridgeBus>) -> BinS2SResult<()> {
        stack.spawn_service::<SourceToTargetHeaderRelayService>()?;
        stack.spawn_service::<TargetToSourceHeaderRelayService>()?;
        stack.spawn_service::<SourceToTargetMessageRelayService>()?;
        stack.spawn_service::<TargetToSourceMessageRelayService>()?;
        Ok(())
    }
}

impl BridgeTask {
    pub fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SubscribeService>()?;
        stack.spawn_service::<FeemarketService>()?;

        #[cfg(feature = "solo-with-solo")]
        Self::spawn_relay_solo_with_solo(&mut stack)?;

        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
