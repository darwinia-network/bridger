use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{BridgeConfig, BridgeTaskBus};

#[derive(Debug)]
pub struct FeemarketService {
    _greet: Lifeline,
}

impl BridgeService for FeemarketService {}

impl Service for FeemarketService {
    type Bus = BridgeTaskBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("feemarket-service", async move { Ok(()) });
        Ok(Self { _greet })
    }
}
