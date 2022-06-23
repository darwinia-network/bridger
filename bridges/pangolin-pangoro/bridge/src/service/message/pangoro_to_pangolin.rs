use abstract_bridge_s2s::strategy::AlwaysRelayStrategy;
use client_pangolin::client::PangolinClient;
use client_pangoro::client::PangoroClient;
use lifeline::{Lifeline, Service, Task};

use relay_s2s::message::{DeliveryRunner, ReceivingRunner};
use relay_s2s::types::{MessageDeliveryInput, MessageReceivingInput};
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PangoroToPangolinMessageRelayService {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
}

impl BridgeService for PangoroToPangolinMessageRelayService {}

impl Service for PangoroToPangolinMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task(
            "pangoro-to-pangolin-message-delivery-service",
            async move {
                while let Err(e) = start_delivery().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[message-relay] [pangoro-to-pangolin] An error occurred for message delivery relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "pangolin-pangoro",
                        "[message-relay] [pangoro-to-pangolin] Try to restart message delivery relay service.",
                    );
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "pangoro-to-pangolin-message-receiving-service",
            async move {
                while let Err(e) = start_receiving().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[message-relay] [pangoro-to-pangolin] An error occurred for message receiving relay {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    tracing::info!(
                        target: "pangolin-pangoro",
                        "[message-relay] [pangoro-to-pangolin] Try to restart message receiving relay service.",
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

async fn message_input() -> color_eyre::Result<MessageReceivingInput<PangoroClient, PangolinClient>>
{
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let relay_config = bridge_config.relay;

    let client_pangoro = bridge_config.pangoro.to_pangoro_client().await?;
    let client_pangolin = bridge_config.pangolin.to_pangolin_client().await?;

    let config_index = bridge_config.index;
    let subquery_pangolin = config_index.to_pangolin_subquery()?;
    let subquery_pangoro = config_index.to_pangoro_subquery()?;

    let lanes = relay_config.raw_lanes();

    let input = MessageReceivingInput {
        lanes,
        relayer_account: client_pangoro.account().account_id().clone(),
        client_source: client_pangoro,
        client_target: client_pangolin,
        subquery_source: subquery_pangoro,
        subquery_target: subquery_pangolin,
    };
    Ok(input)
}

async fn start_delivery() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[message-delivery] [delivery-pangoro-to-pangolin] SERVICE RESTARTING..."
    );
    let input = message_input().await?;
    let input = MessageDeliveryInput {
        lanes: input.lanes,
        relayer_account: input.relayer_account,
        client_source: input.client_source,
        client_target: input.client_target,
        subquery_source: input.subquery_source,
        subquery_target: input.subquery_target,
        relay_strategy: AlwaysRelayStrategy,
    };
    let mut runner = DeliveryRunner::new(input);
    Ok(runner.start().await?)
}

async fn start_receiving() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[message-receiving] [receiving-pangoro-to-pangolin] SERVICE RESTARTING..."
    );
    let input = message_input().await?;
    let mut runner = ReceivingRunner::new(input);
    Ok(runner.start().await?)
}
