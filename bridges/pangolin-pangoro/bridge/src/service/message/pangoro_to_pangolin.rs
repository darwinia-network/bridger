use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;
use crate::service::message::pangoro_to_pangolin::delivery_relay::DeliveryRunner;
use crate::service::message::pangoro_to_pangolin::receiving_relay::ReceivingRunner;

mod delivery_relay;
mod receiving_relay;

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
                while let Err(e) = start_delivery_runner().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[delivery-pangoro-to-pangolin] Failed to start pangoro-to-pangolin message delivery relay, \
                        wait some seconds try again: {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "pangoro-to-pangolin-message-receiving-service",
            async move {
                while let Err(e) = start_receiving_runner().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[receiving-pangoro-to-pangolin] Failed to start pangoro-to-pangolin message confirm relay, \
                        wait some seconds try again: {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
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

async fn start_delivery_runner() -> color_eyre::Result<()> {
    let mut runner = DeliveryRunner::new().await?;
    runner.start().await
}

async fn start_receiving_runner() -> color_eyre::Result<()> {
    let mut runner = ReceivingRunner::new().await?;
    runner.start().await
}
