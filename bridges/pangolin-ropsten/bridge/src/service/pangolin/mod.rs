use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Task};

use component_state::state::BridgeState;
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::PangolinRopstenBus;
use crate::bridge::PangolinRopstenTask;
use crate::bridge::ToExtrinsicsMessage;
use crate::service::pangolin::pangolin_scanner::PangolinScanner;

mod pangolin_scanner;
mod scan_authorities_change_signed_event;
mod scan_schedule_authorities_change_event;
mod scan_schedule_mmr_root_event;
mod types;

#[derive(Debug)]
pub struct PangolinService {
    _greet: Lifeline,
}

impl BridgeService for PangolinService {}

impl lifeline::Service for PangolinService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());
        let tracker = Tracker::new(microkv.clone(), "scan.pangolin");

        let _greet = Self::try_task(
            &format!("{}-service-pangolin-scan", PangolinRopstenTask::name()),
            async move {
                let scanner = PangolinScanner;
                scanner
                    .start(
                        microkv.clone(),
                        tracker.clone(),
                        sender_to_extrinsics.clone(),
                    )
                    .await;
                Ok(())
            },
        );

        Ok(Self { _greet })
    }
}
