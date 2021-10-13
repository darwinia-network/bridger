use std::collections::HashSet;
use std::iter::FromIterator;

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use component_thegraph_liketh::types::TransactionEntity;
use component_thegraph_liketh::TheGraphLikeEthComponent;
use support_tracker::Tracker;

use crate::bus::PangolinRopstenBus;
use crate::config::TaskConfig;
use crate::helpers;
use crate::task::PangolinRopstenTask;

/// Check service
#[derive(Debug)]
pub struct CheckService {
    _greet: Lifeline,
}

impl BridgeService for CheckService {}

impl Service for CheckService {
    type Bus = PangolinRopstenBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::NAME);
        let tracker = Tracker::new(microkv, "scan.ropsten.check");

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-check", PangolinRopstenTask::NAME),
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
        log::error!(
            target: PangolinRopstenTask::NAME,
            "ropsten check err {:#?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run(tracker: &Tracker) -> anyhow::Result<()> {
    log::info!(
        target: PangolinRopstenTask::NAME,
        "ROPSTEN CHECK SERVICE RESTARTING..."
    );

    let component_thegraph_liketh = TheGraphLikeEthComponent::restore::<PangolinRopstenTask>()?;
    let thegraph_liketh = component_thegraph_liketh.component().await?;
    // let task_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;

    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
    // Darwinia client
    let darwinia = component_pangolin_subxt.component().await?;

    loop {
        // todo: put this to config
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let offset = tracker.next().await?;
        let limit = 10;

        let txs = thegraph_liketh
            .query_transactions(limit, offset as u32)
            .await?;

        let blocks: HashSet<u64> =
            HashSet::from_iter(txs.iter().map(|tx| tx.block_number).collect::<Vec<u64>>());

        // start check
        for block in blocks {
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "Check redeem block: {}",
                block,
            );

            let mut all_verified = true;

            let txs_for_block = txs
                .iter()
                .filter(|tx| tx.block_number == block)
                .collect::<Vec<&TransactionEntity>>();
            let mut failed_tx = None;
            for tx in txs_for_block {
                match helpers::is_verified(&darwinia, tx).await {
                    Ok(false) => {
                        all_verified = false;
                        failed_tx = Some(tx.clone());
                        break;
                    }
                    Err(e) => {
                        log::error!(
                            target: PangolinRopstenTask::NAME,
                            "Failed verified redeem. [{}]: {}. {:?}",
                            tx.block,
                            tx.block_hash,
                            e
                        );
                        all_verified = false;
                        failed_tx = Some(tx.clone());
                        break;
                    }
                    _ => {}
                }
            }

            if !all_verified {
                log::trace!(
                    target: PangolinRopstenTask::NAME,
                    "Block {} check failed, reason about: {:?}",
                    block,
                    failed_tx,
                );
                // todo: Set maximum time
                continue;
            }
            verified_blocks.push((offset + limit) - 1);
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "Block {} successes check",
                block,
            );
        }
    }
}
