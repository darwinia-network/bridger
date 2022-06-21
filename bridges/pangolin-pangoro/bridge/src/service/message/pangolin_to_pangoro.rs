use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;
use crate::service::message::pangolin_to_pangoro::delivery_relay::DeliveryRunner;
use crate::service::message::pangolin_to_pangoro::receiving_relay::ReceivingRunner;

#[derive(Debug)]
pub struct PangolinToPangoroMessageRelayService {
    _greet_delivery: Lifeline,
    _greet_receiving: Lifeline,
}

impl BridgeService for PangolinToPangoroMessageRelayService {}

impl Service for PangolinToPangoroMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet_delivery = Self::try_task(
            "pangolin-to-pangoro-message-delivery-service",
            async move {
                while let Err(e) = start_delivery_runner().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[delivery-pangolin-to-pangoro] Failed to start pangolin-to-pangoro message delivery relay, \
                        wait some seconds try again: {:?}",
                        e,
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                Ok(())
            },
        );
        let _greet_receiving = Self::try_task(
            "pangolin-to-pangoro-message-receiving-service",
            async move {
                while let Err(e) = start_receiving_runner().await {
                    tracing::error!(
                        target: "pangolin-pangoro",
                        "[receiving-pangolin-pangoro] Failed to start pangolin-to-pangoro message confirm relay, \
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
    // // todo: restart when error happened
    // let mut runner = DeliveryRunner::new().await?;
    // runner.start().await
}

async fn start_receiving_runner() -> color_eyre::Result<()> {
    // // todo: restart when error happened
    // let mut runner = ReceivingRunner::new().await?;
    // runner.start().await
}
