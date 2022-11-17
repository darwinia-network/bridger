use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_lifeline::service::BridgeService;

use crate::bridge::config::solo_with_solo::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct SubscribeService<
    SCI: S2SSoloBridgeSoloChainInfo,
    TCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet_source: Lifeline,
    _greet_target: Lifeline,
    _source_chain_info: PhantomData<SCI>,
    _target_chain_info: PhantomData<TCI>,
    _subquery_info: PhantomData<SI>,
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    BridgeService for SubscribeService<SCI, TCI, SI>
{
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> Service
    for SubscribeService<SCI, TCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SCI, TCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.source.chain().name(),);

        let _greet_source = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_source(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.source.chain().name(),
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.source.chain().name(),
                );
            }
            Ok(())
        });
        let bridge_config: BridgeConfig<SCI, TCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!("subscribe-{}", config_chain.target.chain().name(),);
        let _greet_target = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start_target(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[subscribe] [{}] failed to start subscribe {:?}",
                    config_chain.target.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[subscribe] [{}] try to restart subscription service.",
                    config_chain.target.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet_source,
            _greet_target,
            _source_chain_info: Default::default(),
            _target_chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    SubscribeService<SCI, TCI, SI>
{
    async fn start_source(bridge_config: BridgeConfig<SCI, TCI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.source.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }

    async fn start_target(bridge_config: BridgeConfig<SCI, TCI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.target.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }
}
