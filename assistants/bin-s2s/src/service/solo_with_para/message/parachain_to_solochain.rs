use std::marker::PhantomData;

use feemarket_s2s::relay::basic::BasicRelayStrategy;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::message::{BridgeParachainDeliveryRunner, BridgeSolochainReceivingRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};

use support_lifeline::service::BridgeService;
use support_toolkit::timecount::TimeCount;

use crate::bridge::config::solo_with_para::BridgeConfig;
use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;
use crate::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
    SubqueryInfo,
};

#[derive(Debug)]
pub struct ParachainToSolochainMessageRelayService<
    SCI: S2SParaBridgeSoloChainInfo,
    RCI: S2SParaBridgeRelayChainInfo,
    PCI: S2SSoloBridgeSoloChainInfo,
    SI: SubqueryInfo,
> {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
    _relaychain_info: PhantomData<RCI>,
    _solochain_info: PhantomData<SCI>,
    _parachain_info: PhantomData<PCI>,
    _subquery_info: PhantomData<SI>,
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > BridgeService for ParachainToSolochainMessageRelayService<SCI, RCI, PCI, SI>
{
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > Service for ParachainToSolochainMessageRelayService<SCI, RCI, PCI, SI>
{
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let bridge_config: BridgeConfig<SCI, RCI, PCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_delivery_name = format!(
            "{}-{}-message-delivery-service",
            config_chain.para.chain().name(),
            config_chain.solo.chain().name(),
        );

        let _greet_delivery = Self::try_task(&task_delivery_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start_delivery(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] an error occurred for message delivery relay {:?}",
                    config_chain.para.chain().name(),
                    config_chain.solo.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[message-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.para.chain().name(),
                        config_chain.solo.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] try to restart message delivery relay service.",
                    config_chain.para.chain().name(),
                    config_chain.solo.chain().name(),
                );
            }
            Ok(())
        });

        let bridge_config: BridgeConfig<SCI, RCI, PCI, SI> = bus.storage().clone_resource()?;
        let config_chain = bridge_config.chain.clone();
        let task_receiving_name = format!(
            "{}-{}-message-receiving-service",
            config_chain.para.chain().name(),
            config_chain.solo.chain().name(),
        );

        let _greet_receiving = Self::try_task(&task_receiving_name, async move {
            let mut timecount = TimeCount::new();
            while let Err(e) = Self::start_receiving(bridge_config.clone()).await {
                tracing::error!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] an error occurred for message receiving relay {:?}",
                    config_chain.para.chain().name(),
                    config_chain.solo.chain().name(),
                    e,
                );
                if let Err(duration) = timecount.plus_and_check() {
                    tokio::time::sleep(duration).await;
                    tracing::error!(
                        target: "bin-s2s",
                        "[message-relay] [{}-to-{}] many errors occurred, wait {} seconds",
                        config_chain.para.chain().name(),
                        config_chain.solo.chain().name(),
                        duration.as_secs(),
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                tracing::info!(
                    target: "bin-s2s",
                    "[message-relay] [{}-to-{}] try to restart message receiving relay service.",
                    config_chain.para.chain().name(),
                    config_chain.solo.chain().name(),
                );
            }
            Ok(())
        });
        Ok(Self {
            _greet_delivery,
            _greet_receiving,
            _relaychain_info: Default::default(),
            _solochain_info: Default::default(),
            _parachain_info: Default::default(),
            _subquery_info: Default::default(),
        })
    }
}

impl<
        SCI: S2SParaBridgeSoloChainInfo,
        RCI: S2SParaBridgeRelayChainInfo,
        PCI: S2SSoloBridgeSoloChainInfo,
        SI: SubqueryInfo,
    > ParachainToSolochainMessageRelayService<SCI, RCI, PCI, SI>
{
    async fn message_input(
        bridge_config: BridgeConfig<SCI, RCI, PCI, SI>,
    ) -> BinS2SResult<
        MessageReceivingInput<
            <PCI as S2SSoloBridgeSoloChainInfo>::Client,
            <SCI as S2SParaBridgeSoloChainInfo>::Client,
        >,
    > {
        let relay_config = bridge_config.relay;
        let config_chain = bridge_config.chain;
        let config_index = bridge_config.index;

        let lanes = relay_config.raw_lanes();

        let input = MessageReceivingInput {
            lanes,
            relayer_account: config_chain.para.account()?,
            client_source: config_chain.para.client().await?,
            client_target: config_chain.solo.client().await?,
            subquery_source: config_index.para.subquery()?,
            subquery_target: config_index.solo.subquery()?,
        };
        Ok(input)
    }

    async fn start_delivery(bridge_config: BridgeConfig<SCI, RCI, PCI, SI>) -> BinS2SResult<()> {
        let config_chain = bridge_config.chain.clone();
        tracing::info!(
            target: "bin-s2s",
            "[message-delivery] [delivery-{}-to-{}] SERVICE RESTARTING...",
            config_chain.para.chain().name(),
            config_chain.solo.chain().name(),
        );
        let config_relay = bridge_config.relay.clone();
        let input = Self::message_input(bridge_config).await?;
        let relay_strategy =
            BasicRelayStrategy::new(input.client_source.clone(), config_chain.para.account()?);
        let input = MessageDeliveryInput {
            lanes: input.lanes,
            nonces_limit: 11,
            relayer_account: input.relayer_account,
            client_source: input.client_source,
            client_target: input.client_target,
            subquery_source: input.subquery_source,
            subquery_target: input.subquery_target,
            relay_block_origin: config_chain.solo.origin_type(),
            relay_strategy,
        };
        let runner = BridgeParachainDeliveryRunner::new(input, config_relay.para_id);
        Ok(runner.start().await?)
    }

    async fn start_receiving(bridge_config: BridgeConfig<SCI, RCI, PCI, SI>) -> BinS2SResult<()> {
        tracing::info!(
            target: "bin-s2s",
            "[message-receiving] [receiving-{}-to-{}] SERVICE RESTARTING...",
            bridge_config.chain.para.chain().name(),
            bridge_config.chain.solo.chain().name(),
        );
        let input = Self::message_input(bridge_config).await?;
        let runner = BridgeSolochainReceivingRunner::new(input);
        Ok(runner.start().await?)
    }
}
