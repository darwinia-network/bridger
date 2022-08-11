use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use subquery::Subquery;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::{BridgeConfig, BridgeTask, PangoroKilnBus};
use crate::service::ecdsa_relay::collected_enough_authorities_change_signatures::CollectedEnoughAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collected_enough_new_message_root_signatures::CollectedEnoughNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::collecting_authorities_change_signatures::CollectingAuthoritiesChangeSignaturesRunner;
use crate::service::ecdsa_relay::collecting_new_message_root_signatures::CollectingNewMessageRootSignaturesRunner;
use crate::service::ecdsa_relay::ecdsa_scanner::EcdsaScanner;
use crate::service::ecdsa_relay::types::EcdsaSource;

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
    type Bus = PangoroKilnBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
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
