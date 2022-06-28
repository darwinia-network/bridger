use client_crab::client::CrabClient;
use client_crab_parachain::client::CrabParachainClient;
use feemarket_s2s::relay::basic::BasicRelayStrategy;
use lifeline::{Lifeline, Service, Task};
use subquery_s2s::types::RelayBlockOrigin;

use relay_s2s::message::{BridgeParachainReceivingRunner, BridgeSolochainDeliveryRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct CrabToCrabParachainMessageRelayService {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
}

impl BridgeService for CrabToCrabParachainMessageRelayService {}

impl Service for CrabToCrabParachainMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task(
            "crab-to-crabparachain-message-delivery-service",
            async move {
                while let Err(e) = start_delivery().await {
                    tracing::error!(
                        target: "crab-crabparachain",
                        "[message-relay] [crab-to-crabparachain] An error occurred for message delivery relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "crab-crabparachain",
                        "[message-relay] [crab-to-crabparachain] Try to restart message delivery relay service.",
                    );
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "crab-to-crabparachain-message-receiving-service",
            async move {
                while let Err(e) = start_receiving().await {
                    tracing::error!(
                        target: "crab-crabparachain",
                        "[message-relay] [crab-to-crabparachain] An error occurred for message receiving relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "crab-crabparachain",
                        "[message-relay] [crab-to-crabparachain] Try to restart message receiving relay service.",
                    );
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_delivery,
            _greet_receiving,
        })
    }
}

async fn message_input(
) -> color_eyre::Result<MessageReceivingInput<CrabClient, CrabParachainClient>> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let relay_config = bridge_config.relay;

    let client_crab = bridge_config.crab.to_crab_client().await?;
    let client_crab_parachain = bridge_config
        .crab_parachain
        .to_crab_parachain_client()
        .await?;

    let config_index = bridge_config.index;
    let subquery_crab = config_index.to_crab_subquery();
    let subquery_crab_parachain = config_index.to_crab_parachain_subquery();

    let lanes = relay_config.raw_lanes();

    let input = MessageReceivingInput {
        lanes,
        relayer_account: client_crab.account().account_id().clone(),
        client_source: client_crab,
        client_target: client_crab_parachain,
        subquery_source: subquery_crab,
        subquery_target: subquery_crab_parachain,
    };
    Ok(input)
}

async fn start_delivery() -> color_eyre::Result<()> {
    tracing::info!(
        target: "crab-crabparachain",
        "[message-delivery] [delivery-crab-to-crabparachain] SERVICE RESTARTING..."
    );
    let input = message_input().await?;
    let relay_strategy = BasicRelayStrategy::new(
        input.client_source.clone(),
        input.client_source.account().account_id().clone(),
    );
    let input = MessageDeliveryInput {
        lanes: input.lanes,
        nonces_limit: 11,
        relayer_account: input.relayer_account,
        client_source: input.client_source,
        client_target: input.client_target,
        subquery_source: input.subquery_source,
        subquery_target: input.subquery_target,
        relay_block_origin: RelayBlockOrigin::BridgeCrabParachain,
        relay_strategy,
    };
    let runner = BridgeSolochainDeliveryRunner::new(input);
    Ok(runner.start().await?)
}

async fn start_receiving() -> color_eyre::Result<()> {
    tracing::info!(
        target: "crab-crabparachain",
        "[message-receiving] [receiving-crab-to-crabparachain] SERVICE RESTARTING..."
    );
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let relay_config = bridge_config.relay;
    let input = message_input().await?;
    let runner = BridgeParachainReceivingRunner::new(input, relay_config.para_id);
    Ok(runner.start().await?)
}
