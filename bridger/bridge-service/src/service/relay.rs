use bee_client::api::{Api, Event};
use bee_client::types::client::ChainTypes;
use bee_client::types::substrate::system::System;
use bridge_component::bee::BeeComponent;
use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::task::BridgeTask;
use codec::{Decode, Encode};
use lifeline::{Bus, Lifeline, Service, Task};
use std::marker::PhantomData;

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

// pub struct RelayService<T: ChainTypes> {
//     bee_component: BeeComponent<T>,
// }
//
// impl<T: ChainTypes> RelayService<T> {
//     pub fn new(bee_component: BeeComponent<T>) -> Self {
//         Self { bee_component }
//     }
// }

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

pub struct RelayService<T: BridgeTask> {
    _greet: Lifeline,
    _marker: PhantomData<T>,
}

impl<T: BridgeTask> Service for RelayService<T> {
    type Bus = T::Bus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        // let mut rx = bus.rx::<BridgerMessage>()?;
        let _greet = Self::try_task("service-ethereum-confirmed", async move {
            // while let Some(recv) = rx.recv().await {
            // 	println!(">>------------------- {:?}", recv);
            // }
            Ok(())
        });
        Ok(Self {
            _greet,
            _marker: Default::default(),
        })
    }
}
