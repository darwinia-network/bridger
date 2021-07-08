use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Lifeline, Receiver, Sender, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_ethereum::web3::Web3Component;
use component_state::state::BridgeState;

use crate::bus::DarwiniaEthereumBus;
use crate::config::SubstrateEthereumConfig;
use crate::message::{DarwiniaEthereumMessage, EthereumScanMessage, ToDarwiniaLinkedMessage};
use crate::task::DarwiniaEthereumTask;

mod ethereum_logs_handler;
use ethereum_logs_handler::EthereumLogsHandler;
use evm_log_tracker::{Ethereum, EvmClient, EvmLogTracker, Result};
use array_bytes::hex2bytes_unchecked as bytes;
use web3::{
    Web3, 
    transports::http::Http,
    types::{Log, H160, H256}
};

fn create_tracker(web3: Web3<Http>, topics_list: Vec<(H160, Vec<H256>)>, scan_from: u64, step: u64) -> EvmLogTracker<Ethereum, EthereumLogsHandler> {
    let client = EvmClient::new(web3);
    EvmLogTracker::<Ethereum, EthereumLogsHandler>::new(
        client,
        topics_list,
        EthereumLogsHandler {},
        100,
        10,
    )
}

fn get_topics_list() -> Vec<(H160, Vec<H256>)> {
    let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
    let contract_address = H160::from_slice(&bytes(contract_address));

    let topics = &vec!["0x96635f5f1b0b05ed7e2265d4e13634378280f038e5a958227d4f383f825c2771"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();
    vec![(contract_address, topics)]
}

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumEthereumScanService {
    _greet: Lifeline,
    tracker: Option<EvmLogTracker<Ethereum, EthereumLogsHandler>>,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumEthereumScanService {}

impl lifeline::Service for LikeDarwiniaWithLikeEthereumEthereumScanService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut tx = bus.tx::<DarwiniaEthereumMessage>()?;
        let mut rx = bus.rx::<DarwiniaEthereumMessage>()?;
        let component_web3 = Web3Component::restore::<DarwiniaEthereumTask>()?;
        let state = bus.storage().clone_resource::<BridgeState>()?;

        let _greet = Self::try_task(
            &format!("{}-service-ethereum-scan", DarwiniaEthereumTask::NAME),
            async move {
                let config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
                let web3 = component_web3.component().await?;
                let microkv = state.microkv();

                let topics_list = get_topics_list();
                let scan_from: u64 = 12345;
                let mut tracker = create_tracker(web3.clone(), topics_list, scan_from, config.interval_ethereum);

                while let Some(recv) = rx.recv().await {
                    if let DarwiniaEthereumMessage::Scan(message_scan) = recv {
                        match message_scan {
                            EthereumScanMessage::Start => {
                                tracker.start().await;
                            }
                            EthereumScanMessage::Pause => {
                                tracker.stop();
                            }
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet, tracker: None })
    }
}
