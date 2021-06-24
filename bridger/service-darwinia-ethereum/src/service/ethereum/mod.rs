use std::marker::PhantomData;

use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_component::Component;
use bridge_config::config::service::SubstrateEthereumConfig;
use bridge_config::Config;
use bridge_standard::bridge::chain::{LikeDarwiniaChain, LikeEthereumChain, SubstrateChain};
use bridge_standard::bridge::component::BridgeComponent;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeTask;

use crate::message::s2e::EthereumScanMessage;

mod scan;

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumEthereumScanService<T: BridgeTask + 'static> {
    _greet: Lifeline,
    _marker: PhantomData<T>,
}

impl<T: BridgeTask + 'static> BridgeService<T>
    for LikeDarwiniaWithLikeEthereumEthereumScanService<T>
{
}

impl<T: BridgeTask + 'static> Service for LikeDarwiniaWithLikeEthereumEthereumScanService<T>
where
    T::Source: LikeDarwiniaChain,
    T::Target: LikeEthereumChain,
{
    type Bus = T::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx_scan = bus.rx::<EthereumScanMessage<T>>()?;
        let component_web3 = Component::web3::<T>()?;
        let component_microkv = Component::microkv::<T>()?;
        let _greet = Self::try_task(&format!("{}-service-ethereum-scan", T::NAME), async move {
            let config: SubstrateEthereumConfig = Config::restore(T::NAME)?;
            let web3 = component_web3.component().await?;
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
                            debug!(target: T::NAME, "ethereum scan ----->");
                            tokio::time::sleep(tokio::time::Duration::from_millis(
                                config.interval_ethereum * 1_000,
                            ))
                            .await;
                        }
                    }
                    EthereumScanMessage::Pause => {
                        running = false;
                    }
                    _ => {
                        anyhow::bail!("Not support operate: {:?}", recv)
                    }
                }
            }
            Ok(())
        });
        Ok(Self {
            _greet,
            _marker: Default::default(),
        })
    }
}
