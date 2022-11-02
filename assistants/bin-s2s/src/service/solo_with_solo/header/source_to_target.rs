use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::SolochainHeaderRunner;
use relay_s2s::types::SolochainHeaderInput;

use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};
use crate::error::BinS2SResult;
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
    type Lifeline = BinS2SResult<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<CI, SI> = bus.storage().clone_resource();
        let config_chain = &bridge_config.chain;
        let task_name = format!(
            "{}-{}-reader-relay-service",
            config_chain.source.chain().name(),
            config_chain.target.chain().name(),
        );

        let _greet = Self::try_task(task_name, async move {
            while let Err(e) = Self::start(bus).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] An error occurred for header relay {:?}",
                    config_chain.source.chain().name(),
                    config_chain.target.chain().name(),
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] Try to restart header relay service.",
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
    async fn start(bus: &BridgeBus) -> BinS2SResult<()> {
        tracing::info!(
            target: "bin-s2s",
            "[header-source-to-target] [source-to-target] SERVICE RESTARTING..."
        );
        // let bridge_config: BridgeConfig<CI, SI> = Config::restore(Names::BridgeDarwiniaCrab)?;
        let bridge_config: BridgeConfig<CI, SI> = bus.storage().clone_resource();
        let relay_config = bridge_config.relay;

        let config_chain = bridge_config.chain;

        let client_source = config_chain.source.client().await?;
        let client_target = config_chain.target.client().await?;

        let config_index = bridge_config.index;
        let subquery_source = config_index.source.subquery()?;

        let chain_name = client_source.chain_name();

        let input = SolochainHeaderInput {
            client_source,
            client_target,
            subquery_source,
            index_origin_type: config_chain.target.origin_type(),
            enable_mandatory: relay_config.enable_mandatory,
        };
        let runner = SolochainHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
