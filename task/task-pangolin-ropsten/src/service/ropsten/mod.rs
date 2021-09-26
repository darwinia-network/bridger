use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::service::ropsten::check::RopstenScanChecker;
use crate::service::ropsten::scan::RopstenScanRunner;
use crate::task::PangolinRopstenTask;
use bridge_traits::bridge::component::BridgeComponent;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;

mod check;
mod ropsten_logs_handler;
mod scan;

#[derive(Debug)]
pub struct RopstenScanService {
    _greet_scan: Lifeline,
    _greet_check: Lifeline,
}

impl BridgeService for RopstenScanService {}

impl lifeline::Service for RopstenScanService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_redeem = bus.tx::<ToRedeemMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker_scan = Tracker::new(microkv, "scan.ropsten");
        tracker_scan.enable_parallel()?;
        let tracker_check = tracker_scan.clone();

        let _greet_scan = Self::try_task(
            &format!("{}-service-ropsten-scan", PangolinRopstenTask::NAME),
            async move {
                RopstenScanRunner::new(
                    tracker_scan,
                    sender_to_relay.clone(),
                    sender_to_redeem.clone(),
                )
                .start()
                .await;
                Ok(())
            },
        );

        let mut receiver_redeem = bus.rx::<ToRedeemMessage>()?;
        let _greet_check = Self::try_task(
            &format!("{}-service-ropsten-check", PangolinRopstenTask::NAME),
            async move {
                let checker = RopstenScanChecker::new(tracker_check);
                while let Some(message) = receiver_redeem.recv().await {
                    if let ToRedeemMessage::Check(txs) = message {
                        checker.push(txs).await;
                    }
                }
                Ok(())
            },
        );
        Ok(Self {
            _greet_scan,
            _greet_check,
        })
    }
}
