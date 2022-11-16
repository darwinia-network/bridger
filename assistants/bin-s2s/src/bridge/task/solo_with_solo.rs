use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::config::solo_with_solo::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::service::feemarket::FeemarketService;
use crate::service::solo_with_solo::SubscribeService;
use crate::service::solo_with_solo::{
    SourceToTargetHeaderRelayService, SourceToTargetMessageRelayService,
    TargetToSourceHeaderRelayService, TargetToSourceMessageRelayService,
};
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct BridgeTask<
    SCI: S2SSoloBridgeSoloChainInfo,
    TCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    stack: TaskStack<BridgeBus>,
    _source_chain_info: PhantomData<SCI>,
    _target_chain_info: PhantomData<TCI>,
    _subquery_info: PhantomData<SI>,
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    BridgeTask<SCI, TCI, SI>
{
    pub fn new(bridge_config: BridgeConfig<SCI, TCI, SI>) -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<SubscribeService<SCI, TCI, SI>>()?;
        stack.spawn_service::<FeemarketService>()?;
        stack.spawn_service::<SourceToTargetHeaderRelayService<SCI, TCI, SI>>()?;
        stack.spawn_service::<TargetToSourceHeaderRelayService<SCI, TCI, SI>>()?;
        stack.spawn_service::<SourceToTargetMessageRelayService<SCI, TCI, SI>>()?;
        stack.spawn_service::<TargetToSourceMessageRelayService<SCI, TCI, SI>>()?;

        Ok(Self {
            stack,
            _source_chain_info: Default::default(),
            _target_chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    BridgeTask<SCI, TCI, SI>
{
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
