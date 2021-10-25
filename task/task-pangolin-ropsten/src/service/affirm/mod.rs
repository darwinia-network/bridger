use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_state::state::BridgeState;
use component_thegraph_liketh::TheGraphLikeEthComponent;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::config::TaskConfig;
use crate::message::{ToExtrinsicsMessage, ToRelayMessage};
use crate::service::affirm::handler::AffirmHandler;
use crate::task::PangolinRopstenTask;

mod handler;

#[derive(Debug)]
pub struct AffirmService {
    _greet_scan: Lifeline,
    _greet_relay: Lifeline,
    _greet_command: Lifeline,
}

impl BridgeService for AffirmService {}

impl Service for AffirmService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // State
        let state = bus.storage().clone_resource::<BridgeState>()?;

        // affirm scan
        let microkv_scan = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker = Tracker::new(microkv_scan, "scan.ropsten.affirm");
        let microkv_scan = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let sender_to_extrinsics_scan = bus.tx::<ToExtrinsicsMessage>()?;
        let _greet_scan = Self::try_task(
            &format!("{}-service-affirm-scan", PangolinRopstenTask::NAME),
            async move {
                start_scan(
                    tracker.clone(),
                    microkv_scan.clone(),
                    sender_to_extrinsics_scan.clone(),
                )
                .await;
                Ok(())
            },
        );

        // affirm relay service
        let sender_to_extrinsics_relay = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv_relay = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let _greet_relay = Self::try_task(
            &format!("{}-service-affirm-relay", PangolinRopstenTask::NAME),
            async move {
                if let Err(e) =
                    handle_affirm_relay(microkv_relay.clone(), sender_to_extrinsics_relay.clone())
                        .await
                {
                    // todo: write log
                    log::error!(
                        target: PangolinRopstenTask::NAME,
                        "Failed to handle affirm relay, err: {:?}",
                        e
                    );
                }
                Ok(())
            },
        );

        // receive affirm command
        let mut rx = bus.rx::<ToRelayMessage>()?;
        let sender_to_extrinsics_command = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv_command = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let _greet_command = Self::try_task(
            &format!("{}-service-affirm-command", PangolinRopstenTask::NAME),
            async move {
                let handler = AffirmHandler::new(
                    microkv_command.clone(),
                    sender_to_extrinsics_command.clone(),
                )
                .await;

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToRelayMessage::EthereumBlockNumber(block_number) => {
                            log::trace!(
                                target: PangolinRopstenTask::NAME,
                                "Received new ethereum block number to affirm: {}",
                                block_number
                            );
                            handler.update_target(block_number)?;
                        }
                    }
                }

                Ok(())
            },
        );
        Ok(Self {
            _greet_scan,
            _greet_relay,
            _greet_command,
        })
    }
}

async fn handle_affirm_relay(
    microkv: NamespaceMicroKV,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) -> anyhow::Result<()> {
    // Config
    let task_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;
    let mut handler = AffirmHandler::new(microkv.clone(), sender_to_extrinsics.clone()).await;
    loop {
        if let Err(err) = handler.affirm().await {
            log::error!(target: PangolinRopstenTask::NAME, "affirm err: {:#?}", err);
            // TODO: Consider the errors more carefully
            // Maybe a websocket err, so wait 10 secs to reconnect.
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            handler = AffirmHandler::new(microkv.clone(), sender_to_extrinsics.clone()).await;
            continue;
        }

        tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_relay)).await;
    }
}

async fn start_scan(
    tracker: Tracker,
    microkv: NamespaceMicroKV,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) {
    while let Err(err) = run_scan(&tracker, microkv.clone(), sender_to_extrinsics.clone()).await {
        log::error!(
            target: PangolinRopstenTask::NAME,
            "Failed to run scan ropsten transaction. err: {:?}",
            err
        );
    }
}

async fn run_scan(
    tracker: &Tracker,
    microkv: NamespaceMicroKV,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) -> anyhow::Result<()> {
    let task_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;

    // the graph
    let component_thegraph_liketh = TheGraphLikeEthComponent::restore::<PangolinRopstenTask>()?;
    let thegraph_liketh = component_thegraph_liketh.component().await?;

    let handler = AffirmHandler::new(microkv, sender_to_extrinsics).await;

    loop {
        let from = tracker.current().await?;
        let limit = 10usize;

        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32)
            .await?;
        if txs.is_empty() {
            tokio::time::sleep(std::time::Duration::from_secs(
                task_config.interval_ethereum,
            ))
            .await;
            continue;
        }

        // Update affirm target
        for tx in &txs {
            let next_block_number = tx.block_number + 1;
            handler.update_target(next_block_number)?;
        }

        let latest = txs.last().unwrap();
        tracker.finish(latest.block_number as usize)?;
        tokio::time::sleep(std::time::Duration::from_secs(
            task_config.interval_ethereum,
        ))
        .await;
    }
}
