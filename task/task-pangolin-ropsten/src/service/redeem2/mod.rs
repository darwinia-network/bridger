use bridge_traits::bridge::component::BridgeComponent;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Channel, Lifeline, Service, Task};
use postage::prelude::Sink;

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use support_tracker::Tracker;
use support_tracker_evm_log::{EvmLogRangeData, LogsHandler};

use crate::bus::PangolinRopstenBus;
use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::service::redeem2::scan::RedeemScanner;
use crate::task::PangolinRopstenTask;
use crate::toolkit;
use crate::toolkit::scanner::RopstenScanner;

/// Redeem service
#[derive(Debug)]
pub struct Redeem2Service {
    _greet: Lifeline,
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

        // Receiver & Sender
        let sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_redeem = bus.tx::<ToRedeemMessage>()?;

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-redeem-scan", PangolinRopstenTask::NAME),
            async move {
                let handler = RedeemHandler::new(
                    tracker_redeem.clone(),
                    sender_to_relay.clone(),
                    sender_to_redeem.clone(),
                );
                let scanner = RopstenScanner::new(tracker_redeem.clone(), handler);
                scanner.start().await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

#[derive(Clone, Debug)]
struct RedeemHandler {
    tracker: Tracker,
    sender_to_relay: <ToRelayMessage::Channel as Channel>::Tx,
    sender_to_redeem: <ToRedeemMessage::Channel as Channel>::Tx,
}

impl RedeemHandler {
    pub fn new(
        tracker: Tracker,
        sender_to_relay: <ToRelayMessage::Channel as Channel>::Tx,
        sender_to_redeem: <ToRedeemMessage::Channel as Channel>::Tx,
    ) -> Self {
        Self {
            tracker,
            sender_to_relay,
            sender_to_redeem,
        }
    }
}

#[async_trait]
impl LogsHandler for RedeemHandler {
    async fn handle(&mut self, data: EvmLogRangeData) -> anyhow::Result<()> {
        let to = data.to();
        let txs = data.transactions();
        if txs.is_empty() {
            self.tracker.finish(to as usize)?;
            return Ok(());
        }
        for tx in &txs {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "{:?} in ethereum block {}",
                &tx.tx_hash,
                &tx.block
            );
            self.sender_to_relay
                .send(ToRelayMessage::EthereumBlockNumber(tx.block + 1))
                .await?;
        }

        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        // Substrate client
        let pangolin = component_pangolin_subxt.component().await?;

        for tx in &txs {
            if toolkit::is_verified(&pangolin, tx)? {
                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "This ethereum tx {:?} has already been redeemed.",
                    tx.enclosed_hash()
                );
                continue;
            }

            log::trace!(
                target: PangolinRopstenTask::NAME,
                "send to redeem service: {:?}",
                &tx.tx_hash
            );
            self.sender_to_redeem
                .send(ToRedeemMessage::EthereumTransaction(tx.clone()))
                .await?;
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "finished to send to redeem: {:?}",
                &tx.tx_hash
            );
        }
        self.tracker.finish(to as usize)?;
        Ok(())
    }
}
