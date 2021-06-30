use lifeline::{Lifeline, Service, Task};

use bridge_component::Component;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;
use chain_darwinia::DarwiniaChain;

use crate::bus::DarwiniaEthereumBus;
use crate::task::DarwiniaEthereumTask;

/*
// fake code

pub struct RelayService<T: ChainTypes> {
    _greet: Lifeline,
}

impl<T: ChainTypes> Service for RelayService<T> {
    type Bus = BridgeBus;
    type Lifeline = anyhow::Result<Self>;
    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let rx = bus.rx::<BridgeMessage>()?;
        let recv = rx.recv().await.unwrap();;
        let config_name = recv.config_name();
        let bee = Component::with(config_name).bee::<T>()?.component().await?;
        let _greet = Self::try_task("service-relay", async move {
            // .. do something
        });
        Ok(Self { _greet })
    }
}

*/

#[derive(Debug)]
pub struct LikeDarwiniaWithLikeEthereumRelayService {
    _greet: Lifeline,
}

impl BridgeService for LikeDarwiniaWithLikeEthereumRelayService {}

impl Service for LikeDarwiniaWithLikeEthereumRelayService {
    type Bus = DarwiniaEthereumBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(_bus: &Self::Bus) -> Self::Lifeline {
        // let mut rx = bus.rx::<BridgerMessage>()?;
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
