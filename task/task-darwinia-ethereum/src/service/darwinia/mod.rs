use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::bus::DarwiniaEthereumBus;
use crate::message::ToExtrinsicsMessage;
use crate::service::darwinia::darwinia_scanner::DarwiniaScanner;
use crate::task::DarwiniaEthereumTask;

mod darwinia_scanner;
mod scan_authorities_change_signed_event;
mod scan_schedule_authorities_change_event;
mod scan_schedule_mmr_root_event;
mod types;

#[derive(Debug)]
pub struct DarwiniaService {
    _greet: Lifeline,
}

impl BridgeService for DarwiniaService {}

impl lifeline::Service for DarwiniaService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let sender_to_extrinsics = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
        let tracker = Tracker::new(microkv.clone(), "scan.darwinia");

        let _greet = Self::try_task(
            &format!("{}-service-darwinia-scan", DarwiniaEthereumTask::NAME),
            async move {
                let scanner = DarwiniaScanner;
                scanner
                    .start(
                        microkv.clone(),
                        tracker.clone(),
                        sender_to_extrinsics.clone()
                    )
                    .await;
                Ok(())
            },
        );

        Ok(Self { _greet })
    }
}
