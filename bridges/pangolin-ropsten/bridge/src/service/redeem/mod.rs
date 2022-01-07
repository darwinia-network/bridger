use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Service, Task};
use postage::broadcast;

use component_state::state::BridgeState;
use component_thegraph_liketh::component::TheGraphLikeEthComponent;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;
use support_tracker::Tracker;

use crate::bridge::PangolinRopstenBus;
use crate::bridge::PangolinRopstenTask;
use crate::bridge::TaskConfig;
use crate::bridge::{PangolinRopstenConfig, ToExtrinsicsMessage};
use crate::service::redeem::handler::RedeemHandler;

mod handler;

#[derive(Debug)]
pub struct RedeemService {
    _greet_scan: Lifeline
}

impl BridgeService for RedeemService {}

impl Service for RedeemService {
    type Bus = PangolinRopstenBus;
    type Lifeline = color_eyre::Result<Self>;

    #[allow(irrefutable_let_patterns)]
    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());
        let tracker = Tracker::new(microkv, "scan.ropsten.redeem");

        // Receiver & Sender
        let sender_to_extrinsics_scan = bus.tx::<ToExtrinsicsMessage>()?;

        let _greet_scan = Self::try_task(
            &format!("{}-service-redeem-scan", PangolinRopstenTask::name()),
            async move {
                start_scan(
                    tracker.clone(),
                    sender_to_extrinsics_scan.clone()
                )
                .await;
                Ok(())
            },
        );

        Ok(Self {
            _greet_scan
        })
    }
}

async fn start_scan(
    tracker: Tracker,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) {
    while let Err(err) = run_scan(
        &tracker,
        sender_to_extrinsics.clone()
    )
    .await
    {
        tracing::error!(
            target: "pangolin-ropsten",
            "[ropsten] redeem err {:?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run_scan(
    tracker: &Tracker,
    sender_to_extrinsics: broadcast::Sender<ToExtrinsicsMessage>,
) -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

    // task config
    let task_config: TaskConfig = bridge_config.task;

    // the graph
    let thegraph_liketh = TheGraphLikeEthComponent::component(bridge_config.thegraph)?;

    let mut handler =
        RedeemHandler::new(sender_to_extrinsics.clone()).await;
    loop {
        let from = tracker.current().await?;
        let limit = 10usize;

        tracing::trace!(
            target: "pangolin-ropsten",
            "[ropsten] Track redeem block: {} and limit: {}",
            from,
            limit
        );
        let txs = thegraph_liketh
            .query_transactions(from as u64, limit as u32)
            .await?;
        if txs.is_empty() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] Not found any transactions to redeem"
            );
            tokio::time::sleep(std::time::Duration::from_secs(
                task_config.interval_ethereum,
            ))
            .await;
            continue;
        }
        tracing::debug!(
            target: "pangolin-ropsten",
            "[ropsten] Found {} transactions wait to redeem",
            txs.len()
        );

        let mut latest_redeem_block_number = None;
        // send transactions to redeem
        for tx in &txs {
            let mut times = 0;
            match handler.redeem(tx.clone()).await {
                Ok(Some(latest)) => {
                    tracing::trace!(
                        target: "pangolin-ropsten",
                        "[ropsten] Change latest redeemed block number to: {}",
                        latest
                    );
                    latest_redeem_block_number = Some(latest);
                }
                Ok(None) => {
                    tracing::trace!(
                        target: "pangolin-ropsten",
                        "[ropsten] Latest redeemed block number is: {:?}",
                        latest_redeem_block_number
                    );
                    break;
                }
                Err(e) => {
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                    times += 1;
                    handler = RedeemHandler::new(sender_to_extrinsics.clone()).await;
                    if times > 10 {
                        tracing::error!(
                            target: "pangolin-ropsten",
                            "[ropsten] Failed to send redeem message. tx: {:?}, err: {:?}",
                            tx,
                            e
                        );
                        break;
                    }
                }
            }
        }

        if latest_redeem_block_number.is_none() {
            tracing::info!(
                target: "pangolin-ropsten",
                "[ropsten] Not have any block redeemed. please wait affirm"
            );
            tokio::time::sleep(std::time::Duration::from_secs(
                task_config.interval_ethereum,
            ))
            .await;
            continue;
        }

        let latest = latest_redeem_block_number.unwrap();
        tracing::info!(
            target: "pangolin-ropsten",
            "[ropsten] Set scan redeem block number to: {}",
            latest
        );
        tracker.finish(latest as usize)?;
        tokio::time::sleep(std::time::Duration::from_secs(
            task_config.interval_ethereum,
        ))
        .await;
    }
}
