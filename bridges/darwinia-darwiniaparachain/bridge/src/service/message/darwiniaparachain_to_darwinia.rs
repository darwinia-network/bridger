use client_darwinia::client::DarwiniaClient;
use client_darwinia_parachain::client::DarwiniaParachainClient;
use feemarket_s2s::relay::basic::BasicRelayStrategy;
use lifeline::{Lifeline, Service, Task};
use relay_s2s::message::{BridgeParachainDeliveryRunner, BridgeSolochainReceivingRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};

use subquery::types::OriginType;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct DarwiniaParachainToDarwiniaMessageRelayService {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
}

impl BridgeService for DarwiniaParachainToDarwiniaMessageRelayService {}

impl Service for DarwiniaParachainToDarwiniaMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task(
            "darwiniaparachain-to-darwinia-message-delivery-service",
            async move {
                while let Err(e) = start_delivery().await {
                    tracing::error!(
                        target: "darwinia-darwiniaparachain",
                        "[message-relay] [darwiniaparachain-to-darwinia] An error occurred for message delivery relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "darwinia-darwiniaparachain",
                        "[message-relay] [darwiniaparachain-to-darwinia] Try to restart message delivery relay service.",
                    );
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "darwiniaparachain-to-darwinia-message-receiving-service",
            async move {
                while let Err(e) = start_receiving().await {
                    tracing::error!(
                        target: "darwinia-darwiniaparachain",
                        "[message-relay] [darwiniaparachain-to-darwinia] An error occurred for message receiving relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "darwinia-darwiniaparachain",
                        "[message-relay] [darwiniaparachain-to-darwinia] Try to restart message receiving relay service.",
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
) -> color_eyre::Result<MessageReceivingInput<DarwiniaParachainClient, DarwiniaClient>> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
    let relay_config = bridge_config.relay;

    let client_darwinia_parachain = bridge_config
        .darwinia_parachain
        .to_darwinia_parachain_client()
        .await?;
    let client_darwinia = bridge_config.darwinia.to_darwinia_client().await?;

    let config_index = bridge_config.index;
    let subquery_darwinia = config_index.to_darwinia_subquery();
    let subquery_darwinia_parachain = config_index.to_darwinia_parachain_subquery();

    let lanes = relay_config.raw_lanes();

    let input = MessageReceivingInput {
        lanes,
        relayer_account: client_darwinia_parachain.account().account_id().clone(),
        client_source: client_darwinia_parachain,
        client_target: client_darwinia,
        subquery_source: subquery_darwinia_parachain,
        subquery_target: subquery_darwinia,
    };
    Ok(input)
}

async fn start_delivery() -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-darwiniaparachain",
        "[message-delivery] [delivery-darwiniaparachain-to-darwinia] SERVICE RESTARTING..."
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
        relay_block_origin: OriginType::BridgeDarwinia,
        relay_strategy,
    };
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
    let relay_config = bridge_config.relay;
    let runner = BridgeParachainDeliveryRunner::new(input, relay_config.para_id);
    Ok(runner.start().await?)
}

async fn start_receiving() -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-darwiniaparachain",
        "[message-receiving] [receiving-darwiniaparachain-to-darwinia] SERVICE RESTARTING..."
    );
    let input = message_input().await?;
    let runner = BridgeSolochainReceivingRunner::new(input);
    Ok(runner.start().await?)
}
