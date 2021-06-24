use std::fmt::Debug;
use std::marker::PhantomData;

use bridge_standard::bridge::task::BridgeTask;
use lifeline::dyn_bus::DynBus;
use lifeline::Message;
use postage::{broadcast, mpsc};

#[derive(Debug, Clone)]
pub enum EthereumScanMessage<T: BridgeTask + Clone + 'static> {
    Scan,
    #[doc(hidden)]
    _Marker(PhantomData<fn() -> T>),
}

impl<T: BridgeTask + Clone + 'static> Message<T::Bus> for EthereumScanMessage<T> {
    type Channel = broadcast::Sender<Self>;
}
