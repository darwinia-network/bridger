use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig, RelayConfig};

#[derive(Debug)]
pub struct PangolinToPangoroMessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToPangoroMessageRelayService {}

impl Service for PangolinToPangoroMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangolin-to-pangoro-message-relay-service", async move {
            while let Err(e) = start().await {
                tracing::error!(
                    target: "pangolin-pangoro",
                    "Failed to start pangoro-to-pangolin message relay, wait some seconds try again: {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

struct MessageRelay {
    relay_config: RelayConfig,
    client_pangolin: PangolinClient,
    client_pangoro: PangoroClient,
}

impl MessageRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let config_pangolin = bridge_config.pangolin;
        let config_pangoro = bridge_config.pangoro;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_pangoro =
            PangoroClientComponent::component(config_pangoro.to_pangoro_client_config()?).await?;
        Ok(Self {
            relay_config: bridge_config.relay,
            client_pangolin,
            client_pangoro,
        })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[message-pangolin-to-pangoro] SERVICE RESTARTING..."
    );
    let mut message_relay = MessageRelay::new().await?;
    loop {
        match run(&message_relay).await {
            Ok(_) => {}
            Err(err) => {
                message_relay = MessageRelay::new().await?;
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[message-pangolin-to-pangoro] Failed to relay header: {:?}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(message_relay: &MessageRelay) -> color_eyre::Result<()> {
    let lane = message_relay
        .relay_config
        .lanes
        .clone()
        .get(0)
        .cloned()
        .ok_or_else(|| BridgerError::Custom("Missing lane id".to_string()))?;
    let client_pangolin = &message_relay.client_pangolin;
    let outbound_lane_data = client_pangolin
        .runtime()
        .storage()
        .bridge_pangoro_messages()
        .outbound_lanes(lane.0, None)
        .await?;
    let next_nonce = outbound_lane_data.oldest_unpruned_nonce;
    // println!("{:?}", outbound_lane_data);
    Ok(())
}
