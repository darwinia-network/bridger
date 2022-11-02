use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::subscribe::SubscribeJustification;
use relay_s2s::types::JustificationInput;

use support_lifeline::error::SupportLifelineResult;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::error::{BinS2SError, BinS2SResult};
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct SubscribeService<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
    _greet_darwinia: Lifeline,
    _greet_crab: Lifeline,
    _chain_info: PhantomData<CI>,
    _subquery_info: PhantomData<SI>,
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeService for SubscribeService<CI, SI> {}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> Service for SubscribeService<CI, SI> {
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let _greet_darwinia = Self::try_task("todo-subscribe-darwinia", async move {
            while let Err(e) = Self::start_source(bridge_config.clone()).await {
                tracing::error!(target: "bin-s2s", "[subscribe] [darwinia] failed to start subscribe {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(target: "bin-s2s", "[subscribe] [crab] try to restart subscription service.");
            }
            Ok(())
        });
        let bridge_config: BridgeConfig<CI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let _greet_crab = Self::try_task("todo-subscribe-crab", async move {
            while let Err(e) = Self::start_target(bridge_config.clone()).await {
                tracing::error!(target: "bin-s2s", "[subscribe] [crab] failed to start subscribe {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(target: "bin-s2s", "[subscribe] [crab] try to restart subscription service.");
            }
            Ok(())
        });
        Ok(Self {
            _greet_darwinia,
            _greet_crab,
            _chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> SubscribeService<CI, SI> {
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
