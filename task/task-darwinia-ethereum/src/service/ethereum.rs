use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Channel, Lifeline, Service, Task};

use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use component_thegraph_liketh::TheGraphLikeEthComponent;
use support_tracker::Tracker;

use crate::bus::DarwiniaEthereumBus;
use crate::config::TaskConfig;
use crate::helpers;
use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::task::DarwiniaEthereumTask;

/// Redeem service
#[derive(Debug)]
pub struct EthereumScanService {
    _greet: Lifeline,
}

impl BridgeService for EthereumScanService {}

impl Service for EthereumScanService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;
        let microkv = state.microkv_with_namespace(DarwiniaEthereumTask::NAME);
        let tracker = Tracker::new(microkv, "scan.ethereum.redeem");
        // Receiver & Sender
        let sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_redeem = bus.tx::<ToRedeemMessage>()?;

        // scan task
        let _greet = Self::try_task(
            &format!("{}-service-redeem", DarwiniaEthereumTask::NAME),
            async move {
                start(
                    tracker.clone(),
                    sender_to_relay.clone(),
                    sender_to_redeem.clone(),
                )
                .await;
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn start(
    tracker: Tracker,
    mut sender_to_relay: <ToRelayMessage::Channel as Channel>::Tx,
    mut sender_to_redeem: <ToRedeemMessage::Channel as Channel>::Tx,
) {
    while let Err(err) = run(&tracker, &mut sender_to_relay, &mut sender_to_redeem).await {
        log::error!(
            target: DarwiniaEthereumTask::NAME,
            "ethereum redeem err {:#?}",
            err
        );
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn run(
    tracker: &Tracker,
    sender_to_relay: &mut <ToRelayMessage::Channel as Channel>::Tx,
    sender_to_redeem: &mut <ToRedeemMessage::Channel as Channel>::Tx,
) -> anyhow::Result<()> {
    log::info!(
        target: DarwiniaEthereumTask::NAME,
        "ETHEREUM SCAN SERVICE RESTARTING..."
    );

    let component_thegraph_liketh = TheGraphLikeEthComponent::restore::<DarwiniaEthereumTask>()?;
    let thegraph_liketh = component_thegraph_liketh.component().await?;
    let task_config: TaskConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;
    // Darwinia client
    let darwinia = component_pangolin_subxt.component().await?;

    loop {
        // fixme: when bridger restarted, may have some transaction not redeemed
        let offset = tracker.current().await?;
        let limit = 10;

        let txs = thegraph_liketh
            .query_transactions(limit, offset as u32)
            .await?;
        // send transactions to relay
        for tx in &txs {
            log::trace!(
                target: DarwiniaEthereumTask::NAME,
                "{:?} in ethereum block {}",
                &tx.tx_hash,
                &tx.block
            );
            // question: why there use tx.blockNumber + 1
            sender_to_relay
                .send(ToRelayMessage::EthereumBlockNumber(tx.block_number + 1))
                .await?;
        }

        // send transactions to redeem
        for tx in &txs {
            if helpers::is_verified(&darwinia, tx)? {
                log::trace!(
                    target: DarwiniaEthereumTask::NAME,
                    "This ethereum tx {:?} has already been redeemed.",
                    tx.enclosed_hash()
                );
                continue;
            }

            log::trace!(
                target: DarwiniaEthereumTask::NAME,
                "send to redeem service: {:?}",
                &tx.tx_hash
            );
            sender_to_redeem
                .send(ToRedeemMessage::EthereumTransaction(tx.clone()))
                .await?;
            log::trace!(
                target: DarwiniaEthereumTask::NAME,
                "finished to send to redeem: {:?}",
                &tx.tx_hash
            );
        }

        tracker.finish(offset + limit)?;
        tokio::time::sleep(std::time::Duration::from_secs(
            task_config.interval_ethereum,
        ))
        .await;
    }
}
