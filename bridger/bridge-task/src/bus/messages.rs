use lifeline::dyn_bus::DynBus;
use lifeline::Message;
use postage::mpsc;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum EthereumMessage<T: DynBus + 'static> {
    Confirmed(u64),
    #[doc(hidden)]
    _Marker(PhantomData<fn() -> T>),
}

impl<T: DynBus + 'static> Message<T> for EthereumMessage<T> {
    type Channel = mpsc::Sender<Self>;
}
