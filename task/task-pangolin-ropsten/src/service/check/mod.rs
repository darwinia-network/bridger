use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::service::check::scan::CheckScanner;
use crate::task::PangolinRopstenTask;

mod scan;

/// Redeem service
#[derive(Debug)]
pub struct CheckService {
    _greet_scan: Lifeline,
    _greet_handler: Lifeline,
}

impl BridgeService for CheckService {}

impl Service for CheckService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker_check = Tracker::new(microkv, "scan.ropsten.check");

        // scan task
        let _greet_scan = Self::try_task(
            &format!("{}-service-checker-scan", PangolinRopstenTask::NAME),
            async move {
                let scanner = CheckScanner::new(tracker_check);
                scanner.start().await;
                Ok(())
            },
        );
        // handler task
        let _greet_handler = Self::try_task(
            &format!("{}-service-checker-handler", PangolinRopstenTask::NAME),
            async move { Ok(()) },
        );
        Ok(Self {
            _greet_scan,
            _greet_handler,
        })
    }
}
