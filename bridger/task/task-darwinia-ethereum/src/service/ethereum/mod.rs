use std::time::Duration;

use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Task};
use microkv::MicroKV;
use postage::broadcast;
use tokio::time::sleep;
use web3::{
    transports::http::Http,
    types::{H160, H256},
    Web3,
};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_darwinia_subxt::darwinia::client::Darwinia;
use component_ethereum::web3::Web3Component;
use component_state::state::BridgeState;
use ethereum_logs_handler::EthereumLogsHandler;
use evm_log_tracker::{Ethereum, EvmClient, EvmLogTracker};

use crate::bus::DarwiniaEthereumBus;
use crate::config::SubstrateEthereumConfig;
use crate::message::{ToRedeemMessage, ToRelayMessage, ToEthereumMessage};
use crate::task::DarwiniaEthereumTask;
use component_ethereum::config::EthereumConfig;

mod ethereum_logs_handler;
use async_recursion::async_recursion;

fn create_tracker(
    darwinia_client: Darwinia,
    microkv: MicroKV,
    sender_to_relay: broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: broadcast::Sender<ToRedeemMessage>,
    web3: Web3<Http>,
    topics_list: Vec<(H160, Vec<H256>)>,
    scan_from: u64,
    step: u64,
) -> EvmLogTracker<Ethereum, EthereumLogsHandler> {
    let client = EvmClient::new(web3);
    let logs_handler = EthereumLogsHandler::new(
        topics_list.clone(),
        sender_to_relay,
        sender_to_redeem,
        microkv,
        darwinia_client,
    );
    EvmLogTracker::<Ethereum, EthereumLogsHandler>::new(
        client,
        topics_list,
        logs_handler,
        scan_from,
        step,
    )
}

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
pub struct LikeDarwiniaWithLikeEthereumEthereumScanService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumEthereumScanService {}

impl lifeline::Service for LikeDarwiniaWithLikeEthereumEthereumScanService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // Receiver & Sender
        let mut rx = bus.rx::<ToEthereumMessage>()?;
        let sender_to_relay = bus.tx::<ToRelayMessage>()?;
        let sender_to_redeem = bus.tx::<ToRedeemMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let mut running = false;
        let _greet = Self::try_task(
            &format!("{}-service-ethereum-scan", DarwiniaEthereumTask::NAME),
            async move {

                while let Some(recv) = rx.recv().await {
                    match recv {
                        ToEthereumMessage::Start => {
                            if !running {
                                running = true;

                                let cloned_state = state.clone();
                                let cloned_sender_to_relay = sender_to_relay.clone();
                                let cloned_sender_to_redeem = sender_to_redeem.clone();
                                tokio::spawn(async move {
                                    run(cloned_state, cloned_sender_to_relay, cloned_sender_to_redeem).await
                                });
                            }
                        }
                        _ => {}
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

#[async_recursion]
async fn run(
    state: BridgeState,
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>
) {
    if let Err(err) = start(state.clone(), sender_to_relay.clone(), sender_to_redeem.clone()).await {
        error!(target: DarwiniaEthereumTask::NAME, "ethereum err {:#?}", err);
        sleep(Duration::from_secs(10)).await;
        run(state, sender_to_relay, sender_to_redeem).await;
    }
}

async fn start(
    state: BridgeState,
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>
) -> anyhow::Result<()> {
    info!(target: DarwiniaEthereumTask::NAME, "SERVICE RESTARTING...");

    // Components
    let component_web3 = Web3Component::restore::<DarwiniaEthereumTask>()?;
    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;

    // Config
    let servce_config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
    let ethereum_config: EthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    let microkv = state.microkv();

    // Web3 client
    let web3 = component_web3.component().await?;

    // Darwinia client
    let darwinia = component_darwinia_subxt.component().await?;

    let topics_list = get_topics_list(ethereum_config);
    let scan_from: u64 = microkv.get("last-redeemed")?.unwrap_or(0) + 1;

    info!(target: DarwiniaEthereumTask::NAME, "✨ SERVICE STARTED: ETHEREUM <> DARWINIA ETHEREUM SUBSCRIBE");

    let mut tracker = create_tracker(
        darwinia,
        microkv.clone(),
        sender_to_relay,
        sender_to_redeem,
        web3.clone(),
        topics_list,
        scan_from,
        servce_config.interval_ethereum,
    );

    tracker.start().await?;

    Ok(())
}
