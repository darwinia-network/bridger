use lifeline::{Bus, Lifeline, Receiver, Sender, Task};

use bridge_component::Component;
use bridge_standard::bridge::component::BridgeComponent;
use bridge_standard::bridge::config::Config;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;

use crate::bus::DarwiniaEthereumBus;
use crate::config::SubstrateEthereumConfig;
use crate::message::darwinia::ToDarwiniaLinkedMessage;
use crate::message::s2e::EthereumScanMessage;
use crate::task::DarwiniaEthereumTask;

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumEthereumScanService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumEthereumScanService {}

impl lifeline::Service for LikeDarwiniaWithLikeEthereumEthereumScanService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut tx_darwinia = bus.tx::<ToDarwiniaLinkedMessage>()?;
        let mut rx_scan = bus.rx::<EthereumScanMessage>()?;
        let component_web3 = Component::web3::<DarwiniaEthereumTask>()?;
        let component_microkv = Component::microkv::<DarwiniaEthereumTask>()?;

        let _greet = Self::try_task(
            &format!("{}-service-ethereum-scan", DarwiniaEthereumTask::NAME),
            async move {
                let config: SubstrateEthereumConfig = Config::restore(DarwiniaEthereumTask::NAME)?;
                let _web3 = component_web3.component().await?;
                let microkv = component_microkv.component().await?;
                let mut running = false;
                while let Some(recv) = rx_scan.recv().await {
                    match recv {
                        EthereumScanMessage::Start => {
                            if running {
                                continue;
                            }
                            running = true;
                            loop {
                                if !running {
                                    break;
                                }
                                debug!(target: DarwiniaEthereumTask::NAME, "ethereum scan ----->");
                                let block_number: u64 = 12345;
                                microkv.put("last_synced", &block_number)?;
                                let las_synced: Option<u64> = microkv.get("last_synced")?;
                                debug!(
                                    target: DarwiniaEthereumTask::NAME,
                                    "Last synced block number is: {:?}", las_synced,
                                );
                                tx_darwinia
                                    .send(ToDarwiniaLinkedMessage::SendExtrinsic)
                                    .await?;
                                tokio::time::sleep(tokio::time::Duration::from_millis(
                                    config.interval_ethereum * 1_000,
                                ))
                                .await;
                            }
                        }
                        EthereumScanMessage::Pause => {
                            running = false;
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
