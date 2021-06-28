use std::marker::PhantomData;

use lifeline::{Lifeline, Service, Task};

use bridge_component::Component;
use bridge_standard::bridge::chain::{LikeDarwiniaChain, LikeEthereumChain, SubstrateChain};
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::shared_service::SharedService;
use bridge_standard::bridge::task::BridgeTask;

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
pub struct LikeDarwiniaWithLikeEthereumRelayService<T: BridgeTask + 'static> {
    _greet: Lifeline,
    _marker: PhantomData<T>,
    // shared_channel: Option<S>,
}

impl<T: BridgeTask + 'static> BridgeService<T> for LikeDarwiniaWithLikeEthereumRelayService<T> {}

impl<T: BridgeTask + 'static> SharedService<T> for LikeDarwiniaWithLikeEthereumRelayService<T> {
    fn spawn_with_shared(bus: &T::Bus) -> anyhow::Result<Self> {
        todo!()
    }
}

impl<T: BridgeTask + 'static> Service for LikeDarwiniaWithLikeEthereumRelayService<T>
where
    T::Source: LikeDarwiniaChain,
    T::Target: LikeEthereumChain,
{
    type Bus = T::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // let mut rx = bus.rx::<BridgerMessage>()?;
        let component_bee = Component::bee::<T, T::Source>()?;
        let _greet = Self::try_task(&format!("{}-service-relay", T::NAME), async move {
            debug!(target: T::NAME, "hello relay");

            // while let Some(recv) = rx.recv().await {
            // 	println!(">>------------------- {:?}", recv);
            // }

            // loop {
            //     println!("hello");
            // }

            Ok(())
        });
        Ok(Self {
            _greet,
            _marker: Default::default(),
            // shared_service: None,
        })
    }
}
