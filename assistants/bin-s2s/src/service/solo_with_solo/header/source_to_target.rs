use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;

use support_lifeline::error::SupportLifelineResult;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::error::{BinS2SError, BinS2SResult};
use crate::traits::{S2SSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct SourceToTargetHeaderRelayService<CI: S2SSoloChainInfo, SI: SubqueryInfo> {
    _greet: Lifeline,
    _chain_info: PhantomData<CI>,
    _subquery_info: PhantomData<SI>,
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> BridgeService
    for SourceToTargetHeaderRelayService<CI, SI>
{
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> Service for SourceToTargetHeaderRelayService<CI, SI> {
    type Bus = BridgeBus;
    type Lifeline = SupportLifelineResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CI, SI> =
            bus.storage().clone_resource().map_err(BinS2SError::from)?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!(
            "{}-{}-header-relay-service",
            config_chain.source.chain().name(),
            config_chain.target.chain().name(),
        );

        let _greet = Self::try_task(&task_name, async move {
            while let Err(e) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] an error occurred for header relay {:?}",
                    config_chain.source.chain().name(),
                    config_chain.target.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] try to restart header relay service.",
                    config_chain.source.chain().name(),
                    config_chain.target.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet,
            _chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<CI: S2SSoloChainInfo, SI: SubqueryInfo> SourceToTargetHeaderRelayService<CI, SI> {
    async fn start(bridge_config: BridgeConfig<CI, SI>) -> BinS2SResult<()> {
        let relay_config = bridge_config.relay;
        let config_chain = bridge_config.chain;
        let config_index = bridge_config.index;
        tracing::info!(
            target: "bin-s2s",
            "[header-{}-to-{}] SERVICE RESTARTING...",
            config_chain.source.chain().name(),
            config_chain.target.chain().name(),
        );

        let input = SolochainHeaderInput {
            client_source: config_chain.source.client().await?,
            client_target: config_chain.target.client().await?,
            subquery_source: config_index.source.subquery()?,
            index_origin_type: config_chain.target.origin_type(),
            enable_mandatory: relay_config.enable_mandatory,
        };
        let runner = SolochainHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
