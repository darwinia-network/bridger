use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::service::feemarket::FeemarketService;
use crate::service::para_with_para::{
    SourceToTargetMessageRelayService, SourceToTargetParaHeadRelayService,
    SourceToTargetRelaychainGrandpaRelayService, SubscribeService,
    TargetToSourceMessageRelayService, TargetToSourceParaHeadRelayService,
    TargetToSourceRelaychainGrandpaRelayService,
};
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct BridgeTask<
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SParaBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    stack: TaskStack<BridgeBus>,
    _source_parachain_info: PhantomData<SPCI>,
    _source_relaychain_info: PhantomData<SRCI>,
    _target_parachain_info: PhantomData<TPCI>,
    _target_relaychain_info: PhantomData<TRCI>,
    _subquery_info: PhantomData<SI>,
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<SRCI, SPCI, TRCI, TPCI, SI>
{
    pub fn new(
        bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>,
    ) -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.bus().store_resource(bridge_config);
        stack.spawn_service::<FeemarketService>()?;
        stack.spawn_service::<SubscribeService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<SourceToTargetParaHeadRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<SourceToTargetRelaychainGrandpaRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<TargetToSourceParaHeadRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<TargetToSourceRelaychainGrandpaRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<SourceToTargetMessageRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;
        stack.spawn_service::<TargetToSourceMessageRelayService<SRCI, SPCI, TRCI, TPCI, SI>>()?;

        Ok(Self {
            stack,
            _source_parachain_info: Default::default(),
            _source_relaychain_info: Default::default(),
            _target_parachain_info: Default::default(),
            _target_relaychain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeTask<SRCI, SPCI, TRCI, TPCI, SI>
{
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
