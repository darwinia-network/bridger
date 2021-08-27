use std::time::Duration;

use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Task};
use tokio::time::sleep;
use web3::types::{H160, H256};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::config::EthereumConfig;
use component_ethereum::web3::Web3Component;
use component_pangolin_subxt::component::DarwiniaSubxtComponent;
use component_state::state::BridgeState;
use ropsten_logs_handler::RopstenLogsHandler;
use support_tracker::Tracker;
use support_tracker_evm_log::{Ethereum, EvmClient, EvmLogTracker};

use crate::bus::PangolinRopstenBus;
use crate::config::TaskConfig;
use crate::message::{ToRedeemMessage, ToRelayMessage};
use crate::task::PangolinRopstenTask;

mod ropsten_logs_handler;

fn get_topics_list(ethereum_config: EthereumConfig) -> Vec<(H160, Vec<H256>)> {
    let topics_setting = vec![
        // bank
        (
            ethereum_config.subscribe_bank_address,
            ethereum_config.subscribe_bank_topics,
        ),
        // issuing
        (
            ethereum_config.subscribe_issuing_address,
            ethereum_config.subscribe_issuing_topics,
        ),
        // relay
        (
            ethereum_config.subscribe_relay_address,
            ethereum_config.subscribe_relay_topics,
        ),
        // backing
        (
            ethereum_config.subscribe_backing_address,
            ethereum_config.subscribe_backing_topics,
        ),
    ];

    topics_setting
        .iter()
        .map(|item| {
            let contract_address = &item.0;
            let contract_address = H160::from_slice(&bytes(contract_address));

            let topics = item.1.iter().map(|t| H256::from_slice(&bytes(t))).collect();
            (contract_address, topics)
        })
        .collect()
}

#[derive(Debug)]
pub struct RopstenScanService {
    _greet: Lifeline,
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
        let tracker = Tracker::new(microkv, "scan.ropsten");
        tracker.reset_current()?;
        tracker.enable_fast_mode()?;

        let _greet = Self::try_task(
            &format!("{}-service-ropsten-scan", PangolinRopstenTask::NAME),
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
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
) {
    loop {
        if let Err(err) = _start(
            tracker.clone(),
            sender_to_relay.clone(),
            sender_to_redeem.clone(),
        )
        .await
        {
            error!(target: PangolinRopstenTask::NAME, "ethereum err {:#?}", err);
            sleep(Duration::from_secs(10)).await;
            if let Err(e) = tracker.reset_current() {
                error!(
                    target: PangolinRopstenTask::NAME,
                    "failed to reset current: {:?}", e
                );
            }
        }
    }
}

async fn _start(
    tracker: Tracker,
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
) -> anyhow::Result<()> {
    info!(
        target: PangolinRopstenTask::NAME,
        "ROPSTEN SCAN SERVICE RESTARTING..."
    );

    // Components
    let component_web3 = Web3Component::restore::<PangolinRopstenTask>()?;
    let component_pangolin_subxt = DarwiniaSubxtComponent::restore::<PangolinRopstenTask>()?;

    // Config
    let servce_config: TaskConfig = Config::restore(PangolinRopstenTask::NAME)?;
    let ethereum_config: EthereumConfig = Config::restore(PangolinRopstenTask::NAME)?;

    // Web3 client
    let web3 = component_web3.component().await?;

    // Darwinia client
    let darwinia = component_pangolin_subxt.component().await?;

    let topics_list = get_topics_list(ethereum_config);

    info!(
        target: PangolinRopstenTask::NAME,
        "✨ SERVICE STARTED: ROPSTEN <> PANGOLIN ROPSTEN SUBSCRIBE"
    );

    let client = EvmClient::new(web3);
    let logs_handler = RopstenLogsHandler::new(
        topics_list.clone(),
        sender_to_relay,
        sender_to_redeem,
        darwinia,
        tracker.clone(),
    );
    let mut tracker_evm_log = EvmLogTracker::<Ethereum, RopstenLogsHandler>::new(
        client,
        topics_list,
        logs_handler,
        tracker,
        servce_config.interval_ethereum,
    );

    tracker_evm_log.start().await?;

    Ok(())
}
