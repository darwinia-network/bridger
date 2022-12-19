use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::service::feemarket::FeemarketService;
use crate::service::solo_with_para::{
    ParaHeadToSolochainRelayService, ParachainToSolochainMessageRelayService,
    RelaychainToSolochainHeaderRelayService, SolochainToParachainHeaderRelayService,
    SolochainToParachainMessageRelayService, SubscribeService,
};
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct BridgeTask<
    SCI: S2SParaBridgeSoloChainInfo,
    RCI: S2SParaBridgeRelayChainInfo,
    PCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    stack: TaskStack<BridgeBus>,
    _relaychain_info: PhantomData<RCI>,
    _solochain_info: PhantomData<SCI>,
    _parachain_info: PhantomData<PCI>,
    _subquery_info: PhantomData<SI>,
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<SCI, RCI, PCI, SI>
{
    pub fn new(bridge_config: BridgeConfig<SCI, RCI, PCI, SI>) -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<SubscribeService<SCI, RCI, PCI, SI>>()?;
        stack.spawn_service::<FeemarketService>()?;
        stack.spawn_service::<SolochainToParachainHeaderRelayService<SCI, RCI, PCI, SI>>()?;
        stack.spawn_service::<RelaychainToSolochainHeaderRelayService<SCI, RCI, PCI, SI>>()?;
        stack.spawn_service::<ParaHeadToSolochainRelayService<SCI, RCI, PCI, SI>>()?;
        stack.spawn_service::<ParachainToSolochainMessageRelayService<SCI, RCI, PCI, SI>>()?;
        stack.spawn_service::<SolochainToParachainMessageRelayService<SCI, RCI, PCI, SI>>()?;

        Ok(Self {
            stack,
            _relaychain_info: Default::default(),
            _solochain_info: Default::default(),
            _parachain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<SCI, RCI, PCI, SI>
{
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
