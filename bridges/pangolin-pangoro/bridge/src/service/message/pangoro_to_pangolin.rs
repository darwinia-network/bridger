use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;
use crate::service::message::pangoro_to_pangolin::delivery_relay::DeliveryRunner;

mod confirm_relay;
mod delivery_relay;

#[derive(Debug)]
pub struct PangoroToPangolinMessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangoroToPangolinMessageRelayService {}

impl Service for PangoroToPangolinMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangoro-to-pangolin-message-delivery-service", async move {
            while let Err(e) = start_delivery_runner().await {
                tracing::error!(
                    target: "pangolin-pangoro",
                    "[delivery-pangoro-to-pangolin] Failed to start pangolin-to-pangoro message relay, \
                    wait some seconds try again: {:?}",
                    e,
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn start_delivery_runner() -> color_eyre::Result<()> {
    let mut runner = DeliveryRunner::new().await?;
    runner.start().await
}
