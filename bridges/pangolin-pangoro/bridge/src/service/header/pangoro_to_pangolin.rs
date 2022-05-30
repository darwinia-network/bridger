use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct PangoroToPangolinHeaderRelayService {
    _greet: Lifeline,
}

impl BridgeService for PangoroToPangolinHeaderRelayService {}

impl Service for PangoroToPangolinHeaderRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("header-relay-service", async move {
            tracing::trace!(
                target: "pangolin-pangoro",
                "[header-relay] test log"
            );
            Ok(())
        });
        Ok(Self { _greet })
    }
}
