use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use support_common::config::Names;
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::{BridgeBus, BridgeTask};
use crate::service::ecdsa_relay::ecdsa_scanner::EcdsaScanner;

mod collected_enough_authorities_change_signatures;
mod collected_enough_new_message_root_signatures;
mod collecting_authorities_change_signatures;
mod collecting_new_message_root_signatures;
mod ecdsa_scanner;
mod types;

#[derive(Debug)]
pub struct ECDSARelayService {
    _greet: Lifeline,
}

impl BridgeService for ECDSARelayService {}

impl Service for ECDSARelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(BridgeTask::name());
        let tracker = Tracker::new(microkv.clone(), "scan.pangoro");
        let _greet = Self::try_task("ecdsa-relay-pangoro-to-kiln", async move {
            let scanner = EcdsaScanner;
            scanner.start(microkv.clone(), tracker.clone()).await;
            Ok(())
        });
        Ok(Self { _greet })
    }
}
