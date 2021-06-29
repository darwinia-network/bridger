use std::fmt::Debug;
use std::marker::PhantomData;

use bridge_standard::bridge::task::BridgeTask;
use lifeline::Message;
use postage::broadcast;

#[derive(Debug, Clone)]
pub enum EthereumScanMessage<T: BridgeTask + Clone + 'static> {
    Start,
    Pause,
    #[doc(hidden)]
    _Marker(PhantomData<fn() -> T>),
}

impl<T: BridgeTask + Clone + 'static> Message<T::Bus> for EthereumScanMessage<T> {
    type Channel = broadcast::Sender<Self>;
}
