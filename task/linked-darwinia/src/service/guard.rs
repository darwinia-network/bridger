use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::DarwiniaLinkedBus;
use crate::task::DarwiniaLinked;

#[derive(Debug)]
pub struct GuardService {
    _greet: Lifeline,
}

impl BridgeService for GuardService {}

impl Service for GuardService {
    type Bus = DarwiniaLinkedBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let _greet = Self::try_task(
            &format!("{}-service-guard", DarwiniaLinked::NAME),
            async move { Ok(()) },
        );
        Ok(Self { _greet })
    }
}
