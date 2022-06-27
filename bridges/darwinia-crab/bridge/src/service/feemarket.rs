use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;

#[derive(Debug)]
pub struct FeemarketService {
    _greet: Lifeline,
}

impl BridgeService for FeemarketService {}

impl Service for FeemarketService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("feemarket-service", async move { Ok(()) });
        Ok(Self { _greet })
    }
}
