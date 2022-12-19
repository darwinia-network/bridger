use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_lifeline::service::BridgeService;

use crate::bridge::config::para_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct SubscribeService<
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SParaBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet_source: Lifeline,
    _greet_target: Lifeline,
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
    > BridgeService for SubscribeService<SRCI, SPCI, TRCI, TPCI, SI>
{
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for SubscribeService<SRCI, SPCI, TRCI, TPCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI> =
            bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.source_relay.chain().name(),);

        let _greet_source = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_source(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.source_relay.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.source_relay.chain().name(),
                );
            }
            Ok(())
        });
        let bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI> =
            bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.target_relay.chain().name(),);
        let _greet_target = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_target(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.target_relay.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.target_relay.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet_source,
            _greet_target,
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
    > SubscribeService<SRCI, SPCI, TRCI, TPCI, SI>
{
    async fn start_source(
        bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>,
    ) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.source_relay.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }

    async fn start_target(
        bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>,
    ) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.target_relay.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }
}
