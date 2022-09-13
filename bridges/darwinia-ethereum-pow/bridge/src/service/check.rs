use std::time::SystemTime;

use client_darwinia::component::DarwiniaClientComponent;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};
use thegraph_liketh::component::TheGraphLikeEthComponent;
use thegraph_liketh::types::LikethChain;

use component_state::state::BridgeState;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::DarwiniaEthereumTask;
use crate::bridge::TaskConfig;
use crate::bridge::{DarwiniaEthereumBus, DarwiniaEthereumConfig};

/// Check service
#[derive(Debug)]
pub struct CheckService {
    _greet: Lifeline,
}

impl BridgeService for CheckService {}

impl Service for CheckService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::name());
        let tracker = Tracker::new(microkv, "scan.ethereum.check");

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-check", DarwiniaEthereumTask::name()),
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
            target: "darwinia-ethereum",
            "[ethereum] [check] ethereum check err {:#?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run(tracker: &Tracker) -> color_eyre::Result<()> {
    tracing::info!(
        target: "darwinia-ethereum",
        "ROPSTEN CHECK SERVICE RESTARTING..."
    );
    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let task_config: TaskConfig = bridge_config.task;

    let thegraph_liketh =
        TheGraphLikeEthComponent::component(bridge_config.thegraph, LikethChain::Ethereum)?;

    // Darwinia client
    let client = DarwiniaClientComponent::component(bridge_config.darwinia).await?;

    let mut timing = SystemTime::now();
    loop {
        let from = tracker.current().await?;
        let limit = 1usize;

        tracing::trace!(
            target: "darwinia-ethereum",
            "[ethereum] [check] Track check block: {} and limit: {}",
            from,
            limit
        );
        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32, false)
            .await?;
        if txs.is_empty() {
            tracing::info!(
                target: "darwinia-ethereum",
                "[ethereum] [check] All transactions checked"
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
                tracing::error!(
                    target: "darwinia-ethereum",
                    "[ethereum] [check] Failed verified redeem. [{}]: {}. {:?}",
                    tx.block_number,
                    tx.block_hash,
                    e
                );
                return Err(e.into());
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
                    target: "darwinia-ethereum",
                    "[ethereum] [check] The transaction {:?}({}) check redeem long time, skipped",
                    tx_hash,
                    tx_index,
                );
                continue;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(task_config.interval_check)).await;
    }
}
