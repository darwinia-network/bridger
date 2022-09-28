use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::{BridgeBus, BridgeTask};
use crate::service::ecdsa_relay::ecdsa_scanner::{EcdsaScanType, EcdsaScanner};

mod collected_enough_authorities_change_signatures;
mod collected_enough_new_message_root_signatures;
mod collecting_authorities_change_signatures;
mod collecting_new_message_root_signatures;
mod ecdsa_scanner;
mod types;

#[derive(Debug)]
pub struct ECDSARelayService {
    _greet_collecting_message: Lifeline,
    _greet_collected_message: Lifeline,
    _greet_collecting_authorities: Lifeline,
    _greet_collected_authorities: Lifeline,
}

impl BridgeService for ECDSARelayService {}

impl Service for ECDSARelayService {
    type Bus = BridgeBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(BridgeTask::name());
        let tracker_collecting_message =
            Tracker::new(microkv.clone(), "scan.darwinia.collecting-message");
        let tracker_collected_message =
            Tracker::new(microkv.clone(), "scan.darwinia.collected-message");
        let tracker_collecting_authorities =
            Tracker::new(microkv.clone(), "scan.darwinia.collecting-authorities");
        let tracker_collected_authorities =
            Tracker::new(microkv, "scan.darwinia.collected-authorities");
        let _greet_collecting_message =
            Self::try_task("darwinia-to-goerli-ecdsa-collecting-message", async move {
                EcdsaScanner
                    .start(
                        tracker_collecting_message.clone(),
                        EcdsaScanType::CollectingMessage,
                    )
                    .await;
                Ok(())
            });
        let _greet_collected_message =
            Self::try_task("darwinia-to-goerli-ecdsa-collected-message", async move {
                EcdsaScanner
                    .start(
                        tracker_collected_message.clone(),
                        EcdsaScanType::CollectedMessage,
                    )
                    .await;
                Ok(())
            });
        let _greet_collecting_authorities = Self::try_task(
            "darwinia-to-goerli-ecdsa-collecting-authorities",
            async move {
                EcdsaScanner
                    .start(
                        tracker_collecting_authorities.clone(),
                        EcdsaScanType::CollectingAuthority,
                    )
                    .await;
                Ok(())
            },
        );
        let _greet_collected_authorities = Self::try_task(
            "darwinia-to-goerli-ecdsa-collected-authorities",
            async move {
                EcdsaScanner
                    .start(
                        tracker_collected_authorities.clone(),
                        EcdsaScanType::CollectedAuthority,
                    )
                    .await;
                Ok(())
            },
        );
        Ok(Self {
            _greet_collecting_message,
            _greet_collected_message,
            _greet_collecting_authorities,
            _greet_collected_authorities,
        })
    }
}
