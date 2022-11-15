use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::error::BinS2SResult;
use crate::service::feemarket::FeemarketService;
use crate::service::solo_with_solo::{
    SourceToTargetHeaderRelayService, SourceToTargetMessageRelayService,
    TargetToSourceHeaderRelayService, TargetToSourceMessageRelayService,
};
use crate::service::subscribe::SubscribeService;
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct BridgeTask<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
    stack: TaskStack<BridgeBus>,
    _chain_info: PhantomData<CI>,
    _subquery_info: PhantomData<SI>,
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeTask<CI, SI> {
    pub fn new(bridge_config: BridgeConfig<CI, SI>) -> BinS2SResult<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<SubscribeService<CI, SI>>()?;
        stack.spawn_service::<FeemarketService>()?;
        stack.spawn_service::<SourceToTargetHeaderRelayService<CI, SI>>()?;
        stack.spawn_service::<TargetToSourceHeaderRelayService<CI, SI>>()?;
        stack.spawn_service::<SourceToTargetMessageRelayService<CI, SI>>()?;
        stack.spawn_service::<TargetToSourceMessageRelayService<CI, SI>>()?;

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
