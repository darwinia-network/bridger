use bridge_standard::external::lifeline::LifelineBus;
use lifeline::Message;
use postage::mpsc;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum EthereumMessage<T: LifelineBus + 'static> {
    Confirmed(u64),
    #[doc(hidden)]
    _Marker(PhantomData<fn() -> T>),
}

impl<T: LifelineBus + 'static> Message<T> for EthereumMessage<T> {
    type Channel = mpsc::Sender<Self>;
}
