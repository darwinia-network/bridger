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

use std::time::Duration;
use tokio::time::sleep;

fn create_tracker(web3: Web3<Http>, topics_list: Vec<(H160, Vec<H256>)>, scan_from: u64, step: u64) -> EvmLogTracker<Ethereum, EthereumLogsHandler> {
    let client = EvmClient::new(web3);
    EvmLogTracker::<Ethereum, EthereumLogsHandler>::new(
        client,
        topics_list.clone(),
        EthereumLogsHandler::new(topics_list),
        100,
        step,
    )
}


// struct Topic {
//     name: String,
//     address: H160,
//     topic: H256,
//     handle: FnMut(Log) -> Result<()>,
// }

// impl Topic {
//     pub fn new(name: String, address: H160, topic: H256, handle: FnMut(Log) -> Result<()>) -> Topic {
//         Topic {
//             name, address, topic, handle
//         }
//     }

//     pub fn belongs_to(&self, log: Log) -> bool {

//     }
// }

fn get_topics_list() -> Vec<(H160, Vec<H256>)> {
    let topics_setting = vec![
        // ring
        (
            "0x9469d013805bffb7d3debe5e7839237e535ec483",
            vec![
                "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10",
            ]
        ),
        // kton
        (
            "0x9f284e1337a815fe77d2ff4ae46544645b20c5ff",
            vec![
                "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10",
            ]
        ),
        // bank
        (
            "0x649fdf6ee483a96e020b889571e93700fbd82d88",
            vec![
                "0xe77bf2fa8a25e63c1e5e29e1b2fcb6586d673931e020c4e3ffede453b830fb12",
            ]
        ),
        // relay
        (
            "0x5cde5Aafeb8E06Ce9e4F94c2406d3B6CB7098E49",
            vec![
                "0x91d6d149c7e5354d1c671fe15a5a3332c47a38e15e8ac0339b24af3c1090690f",
            ]
        ),
        // backing
        (
            "0xd5FC8F2eB94fE6AAdeE91c561818e1fF4ea2C041",
            vec![
                "0x0c403c4583ff520bad94bf49975b3547a573f7157070022cf8c9a023498d4d11",
                "0xf70fbddcb43e433da621898f5f2628b0a644a77a4389ac2580c5b1de06382fe2",
            ]
        ),
    ];

    topics_setting.iter().map(|item| {
        let contract_address = item.0;
        let contract_address = H160::from_slice(&bytes(contract_address));

        let topics = item.1.iter().map(|t| H256::from_slice(&bytes(t))).collect();
        (contract_address, topics)
    }).collect()
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
                let scan_from: u64 = microkv.get("last_synced")?.unwrap_or(0);
                let mut tracker = create_tracker(web3.clone(), topics_list, scan_from, config.interval_ethereum);

                while let Some(recv) = rx.recv().await {
                    if let DarwiniaEthereumMessage::Scan(message_scan) = recv {
                        match message_scan {
                            EthereumScanMessage::Start => {
                                if tracker.running == false {
                                    tracker.start().await;
                                }
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
        Ok(Self { _greet })
    }
}
