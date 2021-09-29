use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::service::redeem2::scan::RedeemScanner;
use crate::task::PangolinRopstenTask;

mod scan;

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
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker_redeem = Tracker::new(microkv, "scan.ropsten.redeem");

        // scan task
        let _greet_scan = Self::try_task(
            &format!("{}-service-redeem-scan", PangolinRopstenTask::NAME),
            async move {
                let scanner = RedeemScanner::new(tracker_redeem);
                scanner.start().await;
                Ok(())
            },
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