use std::marker::PhantomData;

use feemarket_s2s::relay::basic::BasicRelayStrategy;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::message::{BridgeSolochainDeliveryRunner, BridgeSolochainReceivingRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};

use support_lifeline::service::BridgeService;
use support_toolkit::timecount::TimeCount;

use crate::bridge::config::solo_with_solo::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{S2SSoloBridgeSoloChainInfo, SubqueryInfo};

#[derive(Debug)]
pub struct TargetToSourceMessageRelayService<
    SCI: S2SSoloBridgeSoloChainInfo,
    TCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
    _source_chain_info: PhantomData<SCI>,
    _target_chain_info: PhantomData<TCI>,
    _subquery_info: PhantomData<SI>,
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    BridgeService for TargetToSourceMessageRelayService<SCI, TCI, SI>
{
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo> Service
    for TargetToSourceMessageRelayService<SCI, TCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SCI, TCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_delivery_name = format!(
            "{}-{}-message-delivery-service",
            config_chain.target.chain().name(),
            config_chain.source.chain().name(),
        );

        let _greet_delivery = Self::try_task(&task_delivery_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start_delivery(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] an error occurred for message delivery relay {:?}",
                    config_chain.target.chain().name(),
                    config_chain.source.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[message-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.target.chain().name(),
                        config_chain.source.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] try to restart message delivery relay service.",
                    config_chain.target.chain().name(),
                    config_chain.source.chain().name(),
                );
            }
            Ok(())
        });

        let bridge_config: BridgeConfig<SCI, TCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_receiving_name = format!(
            "{}-{}-message-receiving-service",
            config_chain.target.chain().name(),
            config_chain.source.chain().name(),
        );
        let _greet_receiving = Self::try_task(&task_receiving_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start_receiving(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] an error occurred for message receiving relay {:?}",
                    config_chain.target.chain().name(),
                    config_chain.source.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[message-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.target.chain().name(),
                        config_chain.source.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] try to restart message receiving relay service.",
                    config_chain.target.chain().name(),
                    config_chain.source.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet_delivery,
            _greet_receiving,
            _source_chain_info: Default::default(),
            _target_chain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<SCI: S2SSoloBridgeSoloChainInfo, TCI: S2SSoloBridgeSoloChainInfo, SI: SubqueryInfo>
    TargetToSourceMessageRelayService<SCI, TCI, SI>
{
    async fn message_input(
        bridge_config: BridgeConfig<SCI, TCI, SI>,
    ) -> BinS2SResult<
        MessageReceivingInput<
            <TCI as S2SSoloBridgeSoloChainInfo>::Client,
            <SCI as S2SSoloBridgeSoloChainInfo>::Client,
        >,
    > {
        let relay_config = bridge_config.relay;
        let config_chain = bridge_config.chain;
        let config_index = bridge_config.index;

        let lanes = relay_config.raw_lanes();

        let input = MessageReceivingInput {
            lanes,
            relayer_account: config_chain.target.account()?,
            client_source: config_chain.target.client().await?,
            client_target: config_chain.source.client().await?,
            subquery_source: config_index.target.subquery()?,
            subquery_target: config_index.source.subquery()?,
        };
        Ok(input)
    }

    async fn start_delivery(bridge_config: BridgeConfig<SCI, TCI, SI>) -> BinS2SResult<()> {
        tracing::info!(
            target: "bin-s2s",
            "[message-delivery] [delivery-{}-to-{}] SERVICE RESTARTING...",
            bridge_config.chain.target.chain().name(),
            bridge_config.chain.source.chain().name(),
        );
        let config_chain = bridge_config.chain.clone();
        let input = Self::message_input(bridge_config).await?;
        let relay_strategy =
            BasicRelayStrategy::new(input.client_source.clone(), config_chain.target.account()?);
        let input = MessageDeliveryInput {
            lanes: input.lanes,
            nonces_limit: 11,
            relayer_account: input.relayer_account,
            client_source: input.client_source,
            client_target: input.client_target,
            subquery_source: input.subquery_source,
            subquery_target: input.subquery_target,
            relay_block_origin: config_chain.source.origin_type(),
            relay_strategy,
        };
        let runner = BridgeSolochainDeliveryRunner::new(input);
        Ok(runner.start().await?)
    }

    async fn start_receiving(bridge_config: BridgeConfig<SCI, TCI, SI>) -> BinS2SResult<()> {
        tracing::info!(
            target: "bin-s2s",
            "[message-receiving] [receiving-{}-to-{}] SERVICE RESTARTING...",
            bridge_config.chain.target.chain().name(),
            bridge_config.chain.source.chain().name(),
        );
        let input = Self::message_input(bridge_config).await?;
        let runner = BridgeSolochainReceivingRunner::new(input);
        Ok(runner.start().await?)
    }
}
