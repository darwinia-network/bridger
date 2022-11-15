use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;

use support_lifeline::error::SupportLifelineResult;
use support_lifeline::service::BridgeService;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::{BinS2SError, BinS2SResult};
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct SolochainToParachainHeaderRelayService<
    CSI: S2SParaBridgeSoloChainInfo,
    CRI: S2SParaBridgeRelayChainInfo,
    CPI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet: Lifeline,
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
    > BridgeService for SolochainToParachainHeaderRelayService<CSI, CRI, CPI, SI>
{
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for SolochainToParachainHeaderRelayService<CSI, CRI, CPI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CSI, CRI, CPI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!(
            "{}-{}-header-relay-service",
            config_chain.solo.chain().name(),
            config_chain.para.chain().name(),
        );

        let _greet = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] an error occurred for header relay {:?}",
                    config_chain.solo.chain().name(),
                    config_chain.para.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] try to restart header relay service.",
                    config_chain.solo.chain().name(),
                    config_chain.para.chain().name(),
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
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > SolochainToParachainHeaderRelayService<CSI, CRI, CPI, SI>
{
    async fn start(bridge_config: BridgeConfig<CSI, CRI, CPI, SI>) -> BinS2SResult<()> {
        let relay_config = bridge_config.relay;
        let config_chain = bridge_config.chain;
        let config_index = bridge_config.index;
        tracing::info!(
            target: "bin-s2s",
            "[header-{}-to-{}] SERVICE RESTARTING...",
            config_chain.solo.chain().name(),
            config_chain.para.chain().name(),
        );

        let input = SolochainHeaderInput {
            client_source: config_chain.solo.client().await?,
            client_target: config_chain.para.client().await?,
            subquery_source: config_index.solo.subquery()?,
            index_origin_type: config_chain.para.origin_type(),
            enable_mandatory: relay_config.enable_mandatory,
        };
        let runner = SolochainHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
