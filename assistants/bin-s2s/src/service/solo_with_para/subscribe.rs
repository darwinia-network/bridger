use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

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
pub struct SubscribeService<
    CSI: S2SParaBridgeSoloChainInfo,
    CRI: S2SParaBridgeRelayChainInfo,
    CPI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet_solochain: Lifeline,
    _greet_relaychain: Lifeline,
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
    > BridgeService for SubscribeService<CSI, CRI, CPI, SI>
{
}

impl<
        CSI: S2SParaBridgeSoloChainInfo,
        CRI: S2SParaBridgeRelayChainInfo,
        CPI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for SubscribeService<CSI, CRI, CPI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CSI, CRI, CPI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.solo.chain().name(),);

        let _greet_solochain = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_solochain(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.solo.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.solo.chain().name(),
                );
            }
            Ok(())
        });
        let bridge_config: BridgeConfig<CSI, CRI, CPI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.relay.chain().name(),);
        let _greet_relaychain = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_relaychain(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.relay.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.relay.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet_solochain,
            _greet_relaychain,
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
    > SubscribeService<CSI, CRI, CPI, SI>
{
    async fn start_solochain(bridge_config: BridgeConfig<CSI, CRI, CPI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.solo.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }

    async fn start_relaychain(bridge_config: BridgeConfig<CSI, CRI, CPI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.relay.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }
}
