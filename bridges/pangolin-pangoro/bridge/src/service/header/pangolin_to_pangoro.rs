use client_pangolin::client::PangolinClient;
use client_pangolin::component::PangolinClientComponent;
use client_pangoro::client::PangoroClient;
use client_pangoro::component::PangoroClientComponent;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use subquery_s2s::types::BridgeName;
use subquery_s2s::{Subquery, SubqueryComponent};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeConfig, BridgeTaskBus};

#[derive(Debug)]
pub struct PangolinToPangoroHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToPangoroHeaderRelayService {}

impl Service for PangolinToPangoroHeaderRelayService {
    type Bus = BridgeTaskBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangolin-to-pangoro-header-relay-service", async move {
            start().await.map_err(|e| {
                BridgerError::Custom(format!(
                    "Failed to start pangolin-to-pangoro header relay: {:?}",
                    e
                ))
            })?;
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start() -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-pangoro",
        "[header-pangolin-to-pangoro] SERVICE RESTARTING..."
    );
    let mut header_relay = HeaderRelay::new().await?;
    loop {
        match run(&header_relay).await {
            Ok(_) => {}
            Err(err) => {
                if let Some(e) = err.downcast_ref::<client_pangolin::error::ClientError>() {
                    if e.is_restart_need() {
                        tracing::error!(
                            target: "pangolin-pangoro",
                            "[header-pangolin-to-pangoro] Connection Error. Try to resend later: {:?}",
                            e
                        );
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                if let Some(e) = err.downcast_ref::<client_pangoro::error::ClientError>() {
                    if e.is_restart_need() {
                        tracing::error!(
                            target: "pangolin-pangoro",
                            "[header-pangolin-to-pangoro] Connection Error. Try to resend later: {:?}",
                            e
                        );
                        header_relay = HeaderRelay::new().await?;
                    }
                }
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[header-pangolin-to-pangoro] Failed to relay header: {:?}",
                    err
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn run(header_relay: &HeaderRelay) -> color_eyre::Result<()> {
    Ok(())
}

struct HeaderRelay {
    client_pangolin: PangolinClient,
    client_pangoro: PangoroClient,
    subquery_pangolin: Subquery,
}

impl HeaderRelay {
    async fn new() -> color_eyre::Result<Self> {
        let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;

        let config_pangolin = bridge_config.pangolin;
        let config_pangoro = bridge_config.pangoro;

        let client_pangolin =
            PangolinClientComponent::component(config_pangolin.to_pangolin_client_config()?)
                .await?;
        let client_pangoro =
            PangoroClientComponent::component(config_pangoro.to_pangoro_client_config()?).await?;

        let config_index = bridge_config.index;
        let subquery_pangolin =
            SubqueryComponent::component(config_index.pangolin, BridgeName::PangolinPangoro);
        Ok(Self {
            client_pangolin,
            client_pangoro,
            subquery_pangolin,
        })
    }
}
