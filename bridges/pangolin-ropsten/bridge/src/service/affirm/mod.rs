use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use microkv::namespace::NamespaceMicroKV;
use postage::broadcast;
use thegraph_liketh::component::TheGraphLikeEthComponent;

use component_ethereum::web3::Web3Component;
use component_state::state::BridgeState;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::PangolinRopstenTask;
use crate::bridge::{PangolinRopstenBus, PangolinRopstenConfig};
use crate::bridge::{ToExtrinsicsMessage, ToRelayMessage};
use crate::service::affirm::handler::AffirmHandler;

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
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // State
        let state = bus.storage().clone_resource::<BridgeState>()?;

        // affirm scan
        let microkv_scan = state.microkv_with_namespace(PangolinRopstenTask::name());
        let tracker = Tracker::new(microkv_scan, "scan.ropsten.affirm");
        let microkv_scan = state.microkv_with_namespace(PangolinRopstenTask::name());
        let sender_to_extrinsics_scan = bus.tx::<ToExtrinsicsMessage>()?;
        let _greet_scan = Self::try_task(
            &format!("{}-service-affirm-scan", PangolinRopstenTask::name()),
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
        let microkv_relay = state.microkv_with_namespace(PangolinRopstenTask::name());
        let _greet_relay = Self::try_task(
            &format!("{}-service-affirm-relay", PangolinRopstenTask::name()),
            async move {
                if let Err(e) =
                    handle_affirm_relay(microkv_relay.clone(), sender_to_extrinsics_relay.clone())
                        .await
                {
                    tracing::error!(
                        target: "pangolin-ropsten",
                        "[ropsten] [affirm] [handle] Failed to handle affirm relay, err: {:?}",
                        e
                    );
                }
                Ok(())
            },
        );

        // receive affirm command
        let mut rx = bus.rx::<ToRelayMessage>()?;
        let sender_to_extrinsics_command = bus.tx::<ToExtrinsicsMessage>()?;
        let microkv_command = state.microkv_with_namespace(PangolinRopstenTask::name());
        let _greet_command = Self::try_task(
            &format!("{}-service-affirm-command", PangolinRopstenTask::name()),
            async move {
                let handler = AffirmHandler::new(
                    microkv_command.clone(),
                    sender_to_extrinsics_command.clone(),
                )
                .await;

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToRelayMessage::EthereumBlockNumber(block_number) => {
                            tracing::trace!(
                                target: "pangolin-ropsten",
                                "[ropsten] [affirm] [command] Received new ethereum block number to affirm: {}",
                                block_number
                            );
                            if let Err(e) = handler.update_target(block_number) {
                                tracing::error!(target: "pangolin-ropsten", "{:?}", e);
                            }
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
) -> color_eyre::Result<()> {
    // Config
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let task_config = bridge_config.task;

    let mut handler = AffirmHandler::new(microkv.clone(), sender_to_extrinsics.clone()).await;
    loop {
        if let Err(err) = handler.affirm().await {
            tracing::error!(
                target: "pangolin-ropsten",
                "[ropsten] [affirm] [handle] affirm err: {:#?}", err
            );
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
        tracing::error!(
            target: "pangolin-ropsten",
            "[ropsten] [affirm] [scan] Failed to run scan ropsten transaction. err: {:?}",
            err
        );
    }
}

async fn run_scan(
    tracker: &Tracker,
    microkv: NamespaceMicroKV,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let task_config = bridge_config.task;

    // the graph
    let thegraph_liketh = TheGraphLikeEthComponent::component(bridge_config.thegraph)?;
    let web3 = Web3Component::component(bridge_config.web3)?;

    let handler = AffirmHandler::new(microkv, sender_to_extrinsics).await;

    loop {
        let from = tracker.current().await?;
        let limit = 10usize;

        tracing::trace!(
            target: "pangolin-ropsten",
            "[ropsten] [affirm] [scan] Track affirm block: {} and limit: {}",
            from,
            limit
        );
        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32)
            .await?;
        if txs.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] [affirm] [scan] Not found any transactions to affirm"
            );
            tokio::time::sleep(std::time::Duration::from_secs(
                task_config.interval_ethereum,
            ))
            .await;
            continue;
        }

        let last_eth_block_number = web3.eth().block_number().await?.as_u64();

        let latest = txs.last().unwrap();
        let next_block_number = latest.block_number + 1;

        // Waiting for some blocks, to offset the reorg risk
        if last_eth_block_number < next_block_number
            || last_eth_block_number - next_block_number < 20
        {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] [affirm] [scan] Waiting for some blocks, to offset the reorg risk",
            );
            tokio::time::sleep(std::time::Duration::from_secs(
                task_config.interval_ethereum,
            ))
            .await;
            continue;
        }

        handler.update_target(next_block_number)?;
        tracker.finish(latest.block_number as usize)?;
        tokio::time::sleep(std::time::Duration::from_secs(
            task_config.interval_ethereum,
        ))
        .await;
    }
}
