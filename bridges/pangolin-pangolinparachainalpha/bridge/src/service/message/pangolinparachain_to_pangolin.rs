use client_pangolin::client::PangolinClient;
use client_pangolin_parachain::client::PangolinParachainClient;
use lifeline::{Lifeline, Service, Task};

use feemarket_s2s::relay::basic::BasicRelayStrategy;
use relay_s2s::message::{BridgeParachainDeliveryRunner, BridgeSolochainReceivingRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};
use subquery::types::OriginType;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PangolinParachainAlphaToPangolinMessageRelayService {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
}

impl BridgeService for PangolinParachainAlphaToPangolinMessageRelayService {}

impl Service for PangolinParachainAlphaToPangolinMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task(
            "pangolinparachain-to-pangolin-message-delivery-service",
            async move {
                while let Err(e) = start_delivery().await {
                    tracing::error!(
                        target: "pangolin-pangolinparachainalpha",
                        "[message-relay] [pangolinparachain-to-pangolin] An error occurred for message delivery relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "pangolin-pangolinparachainalpha",
                        "[message-relay] [pangolinparachain-to-pangolin] Try to restart message delivery relay service.",
                    );
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "pangolinparachain-to-pangolin-message-receiving-service",
            async move {
                while let Err(e) = start_receiving().await {
                    tracing::error!(
                        target: "pangolin-pangolinparachainalpha",
                        "[message-relay] [pangolinparachain-to-pangolin] An error occurred for message receiving relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "pangolin-pangolinparachainalpha",
                        "[message-relay] [pangolinparachain-to-pangolin] Try to restart message receiving relay service.",
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
) -> color_eyre::Result<MessageReceivingInput<PangolinParachainClient, PangolinClient>> {
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let relay_config = bridge_config.relay;

    let client_pangolin_parachain = bridge_config
        .pangolin_parachain_alpha
        .to_pangolin_parachain_client()
        .await?;
    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;

    let config_index = bridge_config.index;
    let subquery_pangolin = config_index.to_pangolin_subquery();
    let subquery_pangolin_parachain = config_index.to_pangolin_parachain_subquery();

    let lanes = relay_config.raw_lanes();

    let input = MessageReceivingInput {
        lanes,
        relayer_account: client_pangolin_parachain.account().account_id().clone(),
        client_source: client_pangolin_parachain,
        client_target: client_pangolin,
        subquery_source: subquery_pangolin_parachain,
        subquery_target: subquery_pangolin,
    };
    Ok(input)
}

async fn start_delivery() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachainalpha",
        "[message-delivery] [delivery-pangolinparachain-to-pangolin] SERVICE RESTARTING..."
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
        relay_block_origin: OriginType::BridgePangolin,
        relay_strategy,
    };
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let relay_config = bridge_config.relay;
    let runner = BridgeParachainDeliveryRunner::new(input, relay_config.para_id);
    Ok(runner.start().await?)
}

async fn start_receiving() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangolinparachainalpha",
        "[message-receiving] [receiving-pangolinparachain-to-pangolin] SERVICE RESTARTING..."
    );
    let input = message_input().await?;
    let runner = BridgeSolochainReceivingRunner::new(input);
    Ok(runner.start().await?)
}
