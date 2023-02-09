use std::marker::PhantomData;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::header::RelaychainHeaderRunner;
use relay_s2s::types::RelaychainHeaderInput;

use support_lifeline::service::BridgeService;
use support_toolkit::timecount::TimeCount;

use crate::bridge::config::para_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct TargetToSourceRelaychainGrandpaRelayService<
    SRCI: S2SParaBridgeRelayChainInfo,
    SPCI: S2SParaBridgeSoloChainInfo,
    TRCI: S2SParaBridgeRelayChainInfo,
    TPCI: S2SParaBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet: Lifeline,
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
    > BridgeService for TargetToSourceRelaychainGrandpaRelayService<SRCI, SPCI, TRCI, TPCI, SI>
{
}

impl<
        SRCI: S2SParaBridgeRelayChainInfo,
        SPCI: S2SParaBridgeSoloChainInfo,
        TRCI: S2SParaBridgeRelayChainInfo,
        TPCI: S2SParaBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for TargetToSourceRelaychainGrandpaRelayService<SRCI, SPCI, TRCI, TPCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI> =
            bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_name = format!(
            "{}-{}-header-relay-service",
            config_chain.target_relay.chain().name(),
            config_chain.source_para.chain().name(),
        );

        let _greet = Self::try_task(&task_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] an error occurred for header relay {:?}",
                    config_chain.target_relay.chain().name(),
                    config_chain.source_para.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[header-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.target_relay.chain().name(),
                        config_chain.source_para.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[header-relay] [{}-to-{}] try to restart header relay service.",
                    config_chain.target_relay.chain().name(),
                    config_chain.source_para.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet,
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
    > TargetToSourceRelaychainGrandpaRelayService<SRCI, SPCI, TRCI, TPCI, SI>
{
    async fn start(bridge_config: BridgeConfig<SRCI, SPCI, TRCI, TPCI, SI>) -> BinS2SResult<()> {
        let relay_config = bridge_config.relay;
        let config_chain = bridge_config.chain;
        let config_index = bridge_config.index;
        tracing::info!(
            target: "bin-s2s",
            "[header-{}-to-{}] SERVICE RESTARTING...",
            config_chain.target_relay.chain().name(),
            config_chain.source_para.chain().name(),
        );

        let input = RelaychainHeaderInput {
            client_relaychain: config_chain.target_relay.client().await?,
            client_solochain: config_chain.source_para.client().await?,
            subquery_relaychain: config_index.target_relay.subquery()?,
            subquery_parachain: config_index.target_para.subquery()?,
            index_origin_type: config_chain.source_para.origin_type(),
            enable_mandatory: relay_config.enable_mandatory,
        };
        let runner = RelaychainHeaderRunner::new(input);
        Ok(runner.start().await?)
    }
}
