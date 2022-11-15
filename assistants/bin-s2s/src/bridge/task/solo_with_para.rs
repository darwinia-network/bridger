use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::service::feemarket::FeemarketService;
use crate::service::solo_with_para::{
    ParaHeadToSolochainRelayService, RelaychainToSolochainHeaderRelayService,
    SolochainToParachainHeaderRelayService, SubscribeService,
};
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct BridgeTask<
    CSI: S2SParaBridgeSoloChainInfo,
    CRI: S2SParaBridgeRelayChainInfo,
    CPI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    stack: TaskStack<BridgeBus>,
    _relaychain_info: PhantomData<CRI>,
    _solochain_info: PhantomData<CSI>,
    _parachain_info: PhantomData<CPI>,
    _subquery_info: PhantomData<SI>,
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<CSI, CRI, CPI, SI>
{
    pub fn new(bridge_config: BridgeConfig<CSI, CRI, CPI, SI>) -> BinS2SResult<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<SubscribeService<CSI, CRI, CPI, SI>>()?;
        stack.spawn_service::<FeemarketService>()?;
        stack.spawn_service::<SolochainToParachainHeaderRelayService<CSI, CRI, CPI, SI>>()?;
        stack.spawn_service::<RelaychainToSolochainHeaderRelayService<CSI, CRI, CPI, SI>>()?;
        stack.spawn_service::<ParaHeadToSolochainRelayService<CSI, CRI, CPI, SI>>()?;

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
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<CSI, CRI, CPI, SI>
{
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
