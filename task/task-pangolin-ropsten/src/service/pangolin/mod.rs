use bridge_traits::bridge::component::BridgeComponent;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Task};
use postage::broadcast;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::ethereum::EthereumComponent;
use component_pangolin_subxt::account::DarwiniaAccount;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_pangolin_subxt::to_ethereum::Account as ToEthereumAccount;
use component_pangolin_subxt::to_ethereum::Darwinia2Ethereum;
use component_state::state::BridgeState;
use component_subquery::subquery::Subquery;
use component_subquery::SubqueryComponent;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::pangolin::pangolin_scanner::PangolinScanner;
use crate::service::pangolin::scan_authorities_change_signed_event::ScanAuthoritiesChangeSignedEvent;
use crate::service::pangolin::scan_schedule_authorities_change_event::ScanScheduleAuthoritiesChangeEvent;
use crate::service::pangolin::scan_schedule_mmr_root_event::ScanScheduleMMRRootEvent;
use crate::task::PangolinRopstenTask;

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
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker = Tracker::new(microkv, "scan.pangolin");

        let _greet_scan_authorities_change_signed = Self::try_task(
            &format!("{}-service-pangolin-scan", PangolinRopstenTask::NAME),
            async move {
                let mut scanner = PangolinScanner;
                scanner
                    .start(tracker.clone(), sender_to_extrinsics.clone())
                    .await;
                Ok(())
            },
        );

        Ok(Self { _greet })
    }
}
