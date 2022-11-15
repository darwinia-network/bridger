use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::ParaHeaderRunner;
use relay_s2s::types::ParaHeaderInput;

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
pub struct ParaHeadToSolochainRelayService<
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
    > BridgeService for ParaHeadToSolochainRelayService<CSI, CRI, CPI, SI>
{
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for ParaHeadToSolochainRelayService<CSI, CRI, CPI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CSI, CRI, CPI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!(
            "{}-{}-parahead-relay-service",
            config_chain.relay.chain().name(),
            config_chain.solo.chain().name(),
        );

        let _greet = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] an error occurred for header relay {:?}",
                    config_chain.relay.chain().name(),
                    config_chain.solo.chain().name(),
                    e,
                );
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
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > ParaHeadToSolochainRelayService<CSI, CRI, CPI, SI>
{
    async fn start(bridge_config: BridgeConfig<CSI, CRI, CPI, SI>) -> BinS2SResult<()> {
        let relay_config = bridge_config.relay;
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
            para_id: relay_config.para_id,
        };
        let runner = ParaHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
