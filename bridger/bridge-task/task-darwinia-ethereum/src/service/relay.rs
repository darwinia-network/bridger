use lifeline::{Lifeline, Service, Task};

use bridge_component::Component;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;
use chain_darwinia::DarwiniaChain;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumRelayService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumRelayService {}

impl Service for LikeDarwiniaWithLikeEthereumRelayService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        let _component_bee = Component::bee::<DarwiniaEthereumTask, DarwiniaChain>()?;
        let _greet = Self::try_task(
            &format!("{}-service-relay", DarwiniaEthereumTask::NAME),
            async move {
                debug!(target: DarwiniaEthereumTask::NAME, "hello relay");

                // while let Some(recv) = rx.recv().await {
                // 	println!(">>------------------- {:?}", recv);
                // }

                // loop {
                //     println!("hello");
                // }

                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
