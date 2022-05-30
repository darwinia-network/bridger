use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeBus, BridgeConfig};

#[derive(Debug)]
pub struct MessageRelayService {
    _greet: Lifeline,
}

impl BridgeService for MessageRelayService {}

impl Service for MessageRelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("message-relay-service", async move { Ok(()) });
        Ok(Self { _greet })
    }
}
