use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_component::Component;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;
use chain_darwinia::DarwiniaChain;

use crate::bus::DarwiniaLinkedBus;
use crate::message::DarwiniaLinkedMessage;
use crate::task::DarwiniaLinked;

#[derive(Debug)]
pub struct ExtrinsicService {
    _greet: Lifeline,
}

impl BridgeService for ExtrinsicService {}

impl Service for ExtrinsicService {
    type Bus = DarwiniaLinkedBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<DarwiniaLinkedMessage>()?;
        let _component_bee = Component::bee::<DarwiniaLinked, DarwiniaChain>()?;
        let _greet = Self::try_task(
            &format!("{}-service-extrinsic", DarwiniaLinked::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    log::debug!(
                        target: DarwiniaLinked::NAME,
                        "[{}] recv a new extrinsic message: {:?}",
                        DarwiniaLinked::NAME,
                        message
                    );
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
