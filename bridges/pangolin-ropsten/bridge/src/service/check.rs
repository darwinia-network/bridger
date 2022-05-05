use std::time::SystemTime;

use client_pangolin::component::PangolinClientComponent;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use component_state::state::BridgeState;
use component_thegraph_liketh::component::TheGraphLikeEthComponent;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::PangolinRopstenTask;
use crate::bridge::TaskConfig;
use crate::bridge::{PangolinRopstenBus, PangolinRopstenConfig};

/// Check service
#[derive(Debug)]
pub struct CheckService {
    _greet: Lifeline,
}

impl BridgeService for CheckService {}

impl Service for CheckService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());
        let tracker = Tracker::new(microkv, "scan.ropsten.check");

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-check", PangolinRopstenTask::name()),
            async move {
                start(tracker.clone()).await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(tracker: Tracker) {
    while let Err(err) = run(&tracker).await {
        tracing::error!(
            target: "pangolin-ropsten",
            "[ropsten] [check] ropsten check err {:#?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run(tracker: &Tracker) -> color_eyre::Result<()> {
    tracing::info!(
        target: "pangolin-ropsten",
        "ROPSTEN CHECK SERVICE RESTARTING..."
    );
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let task_config: TaskConfig = bridge_config.task;

    let thegraph_liketh = TheGraphLikeEthComponent::component(bridge_config.thegraph)?;

    // Pangolin client
    let client = PangolinClientComponent::component(bridge_config.darwinia).await?;

    let mut timing = SystemTime::now();
    loop {
        let from = tracker.current().await?;
        let limit = 1usize;

        tracing::trace!(
            target: "pangolin-ropsten",
            "[ropsten] [check] Track check block: {} and limit: {}",
            from,
            limit
        );
        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32)
            .await?;
        if txs.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] [check] All transactions checked"
            );
            tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_check)).await;
            continue;
        }
        let tx = txs.get(0).unwrap();

        let tx_hash = array_bytes::hex2bytes(&tx.block_hash).map_err(|_e| {
            BridgerError::Hex(format!(
                "Failed to convert hex({}) to bytes.",
                &tx.block_hash
            ))
        })?;
        let tx_index = tx.tx_index;
        let verified = match client.ethereum().is_verified(&tx_hash, tx_index).await {
            Ok(v) => v,
            Err(e) => {
                if e.is_restart_need() {
                    return Err(e.into());
                }
                tracing::error!(
                    target: "pangolin-ropsten",
                    "[ropsten] [check] Failed verified redeem. [{}]: {}. {:?}",
                    tx.block_number,
                    tx.block_hash,
                    e
                );
                false
            }
        };
        if verified {
            tracker.finish(tx.block_number as usize)?;
            timing = SystemTime::now();
            continue;
        }

        if let Ok(elapsed) = timing.elapsed() {
            let secs = elapsed.as_secs();
            if secs >= task_config.check_timeout {
                tracker.finish(tx.block_number as usize)?;
                tracing::warn!(
                    target: "pangolin-ropsten",
                    "[ropsten] [check] The transaction {:?}({}) check redeem long time, skipped",
                    tx_hash,
                    tx_index,
                );
                continue;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_check)).await;
    }
}
