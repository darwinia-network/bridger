use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::error::BinS2SResult;
use crate::service::feemarket::FeemarketService;
#[cfg(feature = "solo-with-solo")]
use crate::service::solo_with_solo::SourceToTargetHeaderRelayService;
use crate::service::subscribe::SubscribeService;
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct BridgeTask<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
    stack: TaskStack<BridgeBus>,
    _chain_info: PhantomData<CI>,
    _subquery_info: PhantomData<SI>,
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeTask<CI, SI> {
    pub fn name() -> &'static str {
        "darwinia-crab"
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeTask<CI, SI> {
    #[cfg(feature = "solo-with-solo")]
    fn spawn_relay_solo_with_solo(stack: &mut TaskStack<BridgeBus>) -> BinS2SResult<()> {
        stack.spawn_service::<SourceToTargetHeaderRelayService>()?;
        // stack.spawn_service::<TargetToSourceHeaderRelayService>()?;
        // stack.spawn_service::<SourceToTargetMessageRelayService>()?;
        // stack.spawn_service::<TargetToSourceMessageRelayService>()?;
        Ok(())
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeTask<CI, SI> {
    pub fn new(bridge_config: BridgeConfig<CI, SI>) -> BinS2SResult<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SubscribeService>()?;
        stack.spawn_service::<FeemarketService>()?;
        stack.bus().store_resource(bridge_config);

        #[cfg(feature = "solo-with-solo")]
        Self::spawn_relay_solo_with_solo(&mut stack)?;

        Ok(Self {
            stack,
            _chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeTask<CI, SI> {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
