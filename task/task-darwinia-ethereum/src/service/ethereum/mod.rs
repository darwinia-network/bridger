use std::time::Duration;

use array_bytes::hex2bytes_unchecked as bytes;
use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Sender, Task};
use tokio::time::sleep;
use web3::types::{H160, H256};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;
use component_ethereum::config::EthereumConfig;
use component_ethereum::web3::Web3Component;
use component_state::state::BridgeState;
use ethereum_logs_handler::EthereumLogsHandler;
use evm_log_tracker::{Ethereum, EvmClient, EvmLogTracker};

use crate::bus::DarwiniaEthereumBus;
use crate::config::TaskConfig;
use crate::message::{ToEthereumMessage, ToRedeemMessage, ToRelayMessage};
use crate::task::DarwiniaEthereumTask;

mod ethereum_logs_handler;

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
        let sender_to_ethereum = bus.tx::<ToEthereumMessage>()?;

        // Datastore
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let service_name = format!("{}-service-ethereum-scan", DarwiniaEthereumTask::NAME);
        let _greet = Self::try_task(&service_name.clone(), async move {
            let mut is_started = false;
            while let Some(ToEthereumMessage::Start) = rx.recv().await {
                if is_started {
                    log::warn!("The service {} has been started", service_name.clone());
                    return Ok(());
                }

                let cloned_state = state.clone();
                let cloned_sender_to_relay = sender_to_relay.clone();
                let cloned_sender_to_redeem = sender_to_redeem.clone();
                let cloned_sender_to_ethereum = sender_to_ethereum.clone();
                tokio::spawn(async move {
                    run(
                        cloned_state,
                        cloned_sender_to_relay,
                        cloned_sender_to_redeem,
                        cloned_sender_to_ethereum,
                    )
                    .await
                });
                is_started = true;
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}

async fn run(
    state: BridgeState,
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
    mut sender_to_ethereum: postage::broadcast::Sender<ToEthereumMessage>,
) {
    if let Err(err) = start(
        state.clone(),
        sender_to_relay.clone(),
        sender_to_redeem.clone(),
    )
    .await
    {
        error!(
            target: DarwiniaEthereumTask::NAME,
            "ethereum err {:#?}", err
        );
        sleep(Duration::from_secs(10)).await;
        sender_to_ethereum
            .send(ToEthereumMessage::Start)
            .await
            .unwrap();
    }
}

async fn start(
    state: BridgeState,
    sender_to_relay: postage::broadcast::Sender<ToRelayMessage>,
    sender_to_redeem: postage::broadcast::Sender<ToRedeemMessage>,
) -> anyhow::Result<()> {
    info!(target: DarwiniaEthereumTask::NAME, "SERVICE RESTARTING...");

    // Components
    let component_web3 = Web3Component::restore::<DarwiniaEthereumTask>()?;
    let component_darwinia_subxt = DarwiniaSubxtComponent::restore::<DarwiniaEthereumTask>()?;

    // Config
    let servce_config: TaskConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
    let ethereum_config: EthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;

    let microkv = state.microkv();

    // Web3 client
    let web3 = component_web3.component().await?;

    // Darwinia client
    let darwinia = component_darwinia_subxt.component().await?;

    let topics_list = get_topics_list(ethereum_config);

    info!(
        target: DarwiniaEthereumTask::NAME,
        "✨ SERVICE STARTED: ETHEREUM <> DARWINIA ETHEREUM SUBSCRIBE"
    );

    let client = EvmClient::new(web3);
    let logs_handler = EthereumLogsHandler::new(
        topics_list.clone(),
        sender_to_relay,
        sender_to_redeem,
        microkv.clone(),
        darwinia,
    );
    let mut tracker = EvmLogTracker::<Ethereum, EthereumLogsHandler>::new(
        client,
        topics_list,
        logs_handler,
        state.clone(),
        "last-redeemed".to_string(),
        servce_config.interval_ethereum,
    );

    tracker.start().await?;

    Ok(())
}
