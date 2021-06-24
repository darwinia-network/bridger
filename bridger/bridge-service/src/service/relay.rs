use std::marker::PhantomData;

use bee_client::types::substrate::system::System;
use codec::{Decode, Encode};
use lifeline::{Lifeline, Service, Task};

use bridge_component::Component;
use bridge_standard::bridge::chain::{LikeEthereumChain, SubstrateChain};
use bridge_standard::bridge::task::BridgeTask;

pub trait EthereumRelay: System {
    /// RingBalance
    type RingBalance: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum BlockNumber
    type EthereumBlockNumber: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Pending Header
    type PendingRelayHeaderParcel: 'static + Encode + Decode + Sync + Send + Default;
    /// Ethereum Relay Header ID
    type RelayAffirmationId: 'static + Encode + Decode + Sync + Send + Default + Clone;
}

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

pub struct SubstrateToEthereumRelayService<T: BridgeTask + 'static> {
    _greet: Lifeline,
    _marker: PhantomData<T>,
}

impl<T: BridgeTask + 'static> Service for SubstrateToEthereumRelayService<T>
where
    T::Source: SubstrateChain,
    T::Target: LikeEthereumChain,
{
    type Bus = T::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // let mut rx = bus.rx::<BridgerMessage>()?;
        let component_bee = Component::bee::<T, T::Source>()?;
        let _greet = Self::try_task(&format!("{}-service-relay", T::NAME), async move {
            println!("hello relay");

            // while let Some(recv) = rx.recv().await {
            // 	println!(">>------------------- {:?}", recv);
            // }

            // loop {
            //     println!("hello");
            // }

            Ok(())
        });
        println!("{:?}", _greet);
        Ok(Self {
            _greet,
            _marker: Default::default(),
        })
    }
}
