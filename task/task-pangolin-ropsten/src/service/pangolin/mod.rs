use bridge_traits::bridge::component::BridgeComponent;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Task};
use postage::broadcast;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use component_subquery::subquery::Subquery;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::pangolin::scan_authorities_change_signed_event::ScanAuthoritiesChangeSignedEvent;
use crate::service::pangolin::scan_schedule_authorities_change_event::ScanScheduleAuthoritiesChangeEvent;
use crate::service::pangolin::scan_schedule_mmr_root_event::ScanScheduleMMRRootEvent;
use crate::task::PangolinRopstenTask;

mod scan_authorities_change_signed_event;
mod scan_schedule_authorities_change_event;
mod scan_schedule_mmr_root_event;

#[derive(Debug)]
pub struct PangolinService {
    _greet_scan_authorities_change_signed: Lifeline,
    _greet_scan_schedule_authorities_change: Lifeline,
    _greet_scan_schedule_mmr_root: Lifeline,
}

impl BridgeService for PangolinService {}

impl lifeline::Service for PangolinService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;

        // scan-authorities-change-signed
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);

        let _greet_scan_authorities_change_signed = Self::try_task(
            &format!(
                "{}-service-pangolin-scan-authorities-change-signed",
                PangolinRopstenTask::NAME
            ),
            async move {
                let scanner =
                    ScanAuthoritiesChangeSignedEvent::new(sender_to_extrinsics.clone(), microkv);
                scanner.start().await;
                Ok(())
            },
        );

        // scan-schedule-authorities-change
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);

        let _greet_scan_schedule_authorities_change = Self::try_task(
            &format!(
                "{}-service-pangolin-scan-schedule-authorities-change",
                PangolinRopstenTask::NAME
            ),
            async move {
                let scanner =
                    ScanScheduleAuthoritiesChangeEvent::new(sender_to_extrinsics.clone(), microkv);
                scanner.start().await;
                Ok(())
            },
        );

        // scan-schedule-mmr-root
        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);

        let _greet_scan_schedule_mmr_root = Self::try_task(
            &format!(
                "{}-service-pangolin-scan-schedule-mmr-root",
                PangolinRopstenTask::NAME
            ),
            async move {
                let scanner = ScanScheduleMMRRootEvent::new(sender_to_extrinsics.clone(), microkv);
                scanner.start().await;
                Ok(())
            },
        );

        Ok(Self {
            _greet_scan_authorities_change_signed,
            _greet_scan_schedule_authorities_change,
            _greet_scan_schedule_mmr_root,
        })
    }
}
