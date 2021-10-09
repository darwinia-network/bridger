use std::collections::HashSet;
use std::iter::FromIterator;

use lifeline::dyn_bus::DynBus;
use lifeline::{Lifeline, Service, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use support_ethereum::transaction::EthereumTransaction;
use support_tracker::Tracker;
use support_tracker_evm_log::{EvmLogRangeData, LogsHandler};

use crate::bus::PangolinRopstenBus;
use crate::service::check::scan::CheckScanner;
use crate::task::PangolinRopstenTask;
use crate::toolkit;
use crate::toolkit::scanner::RopstenScanner;

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
            &format!("{}-service-checker", PangolinRopstenTask::NAME),
            async move {
                let handler = CheckHandler::new(tracker.clone());
                let scanner = RopstenScanner::new(tracker.clone(), handler);
                scanner.start().await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

#[derive(Clone, Debug)]
struct CheckHandler {
    tracker: Tracker,
    times: u32,
}

impl CheckHandler {
    pub fn new(tracker: Tracker) -> Self {
        Self { tracker, times: 0 }
    }
}

#[async_trait]
impl LogsHandler for CheckHandler {
    async fn handle(&mut self, data: EvmLogRangeData) -> anyhow::Result<()> {
        self.times += 1;
        let txs = data.transactions();
        if txs.is_empty() {
            self.tracker.finish(to as usize)?;
            self.times = 0;
            return Ok(());
        }
        let blocks: HashSet<u64> =
            HashSet::from_iter(txs.iter().map(|tx| tx.block).collect::<Vec<u64>>());
        let mut verified_blocks = vec![];

        let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;
        // Substrate client
        let client = component_pangolin_subxt.component().await?;

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
                .filter(|tx| tx.block == block)
                .collect::<Vec<&EthereumTransaction>>();
            let mut failed_tx = None;
            for tx in txs_for_block {
                match toolkit::is_verified(&client, tx).await {
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
                continue;
            }
            verified_blocks.push(block);
            log::trace!(
                target: PangolinRopstenTask::NAME,
                "Block {} successes check",
                block,
            );
        }

        todo!()
    }
}
