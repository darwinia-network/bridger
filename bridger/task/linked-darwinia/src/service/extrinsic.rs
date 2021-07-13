use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_darwinia_subxt::component::DarwiniaSubxtComponent;

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
        let _component_darwinia = DarwiniaSubxtComponent::restore::<DarwiniaLinked>()?;
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
