use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::PangolinRopstenBus;
use crate::task::PangolinRopstenTask;

/// Redeem service
#[derive(Debug)]
pub struct Redeem2Service {
    _greet_scan: Lifeline,
    _greet_handler: Lifeline,
}

impl BridgeService for Redeem2Service {}

impl Service for Redeem2Service {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // scan task
        let _greet_scan = Self::try_task(
            &format!("{}-service-redeem-scan", PangolinRopstenTask::NAME),
            async move { Ok(()) },
        );
        // handler task
        let _greet_handler = Self::try_task(
            &format!("{}-service-redeem-handler", PangolinRopstenTask::NAME),
            async move { Ok(()) },
        );
        Ok(Self {
            _greet_scan,
            _greet_handler,
        })
    }
}
