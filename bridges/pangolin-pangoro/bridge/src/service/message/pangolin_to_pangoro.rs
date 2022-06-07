use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;

#[derive(Debug)]
pub struct PangolinToPangoroMessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangolinToPangoroMessageRelayService {}

impl Service for PangolinToPangoroMessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("pangolin-to-pangoro-message-relay-service", async move {
            // while let Err(e) = start().await {
            //     tracing::error!(
            //         target: "pangolin-pangoro",
            //         "Failed to start pangoro-to-pangolin message relay, wait some seconds try again: {:?}",
            //         e,
            //     );
            //     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            // }
            Ok(())
        });
        Ok(Self { _greet })
    }
}
