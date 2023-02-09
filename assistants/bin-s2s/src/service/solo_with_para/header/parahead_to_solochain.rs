use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::ParaHeaderRunner;
use relay_s2s::types::ParaHeaderInput;

use support_lifeline::service::BridgeService;
use support_toolkit::timecount::TimeCount;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct ParaHeadToSolochainRelayService<
    SCI: S2SParaBridgeSoloChainInfo,
    RCI: S2SParaBridgeRelayChainInfo,
    PCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet: Lifeline,
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
    > BridgeService for ParaHeadToSolochainRelayService<SCI, RCI, PCI, SI>
{
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for ParaHeadToSolochainRelayService<SCI, RCI, PCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SCI, RCI, PCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!(
            "{}-{}-parahead-relay-service",
            config_chain.relay.chain().name(),
            config_chain.solo.chain().name(),
        );

        let _greet = Self::try_task(&task_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] an error occurred for header relay {:?}",
                    config_chain.relay.chain().name(),
                    config_chain.solo.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[header-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.relay.chain().name(),
                        config_chain.solo.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] try to restart header relay service.",
                    config_chain.relay.chain().name(),
                    config_chain.solo.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet,
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
    > ParaHeadToSolochainRelayService<SCI, RCI, PCI, SI>
{
    async fn start(bridge_config: BridgeConfig<SCI, RCI, PCI, SI>) -> BinS2SResult<()> {
        let para_config = bridge_config.para_config;
        let config_chain = bridge_config.chain;
        tracing::info!(
            target: "bin-s2s",
            "[header-{}-to-{}] SERVICE RESTARTING...",
            config_chain.relay.chain().name(),
            config_chain.solo.chain().name(),
        );

        let input = ParaHeaderInput {
            client_relaychain: config_chain.relay.client().await?,
            client_solochain: config_chain.solo.client().await?,
            para_id: para_config.para_id,
        };
        let runner = ParaHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
