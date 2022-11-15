use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_lifeline::error::SupportLifelineResult;
use support_lifeline::service::BridgeService;

use crate::bridge::config::solo_with_solo::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::{BinS2SError, BinS2SResult};
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct SubscribeService<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> {
    _greet_source: Lifeline,
    _greet_target: Lifeline,
    _chain_info: PhantomData<CI>,
    _subquery_info: PhantomData<SI>,
}

impl<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> BridgeService for SubscribeService<CI, SI> {}

impl<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> Service for SubscribeService<CI, SI> {
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
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
        let bridge_config: BridgeConfig<CI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
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
            _chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<CI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> SubscribeService<CI, SI> {
    async fn start_source(bridge_config: BridgeConfig<CI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.source.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }

    async fn start_target(bridge_config: BridgeConfig<CI, SI>) -> BinS2SResult<()> {
        let config_chain = &bridge_config.chain;
        let client = config_chain.target.client().await?;

        let input = JustificationInput { client };
        let subscribe = SubscribeJustification::new(input);
        subscribe.start().await?;
        Ok(())
    }
}
