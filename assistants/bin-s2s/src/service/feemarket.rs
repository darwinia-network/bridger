use lifeline::{Lifeline, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::BridgeBus;
use crate::error::BinS2SResult;

#[derive(Debug)]
pub struct FeemarketService {
    _greet: Lifeline,
}

impl BridgeService for FeemarketService {}

impl Service for FeemarketService {
    type Bus = BridgeBus;
    type Lifeline = BinS2SResult<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task("feemarket-service", async move { Ok(()) });
        Ok(Self { _greet })
    }
}
