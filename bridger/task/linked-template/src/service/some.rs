use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_http_client::HttpClientComponent;

use crate::bus::TemplateLinkedBus;
use crate::message::TemplateLinkedMessage;
use crate::task::TemplateLinked;

#[derive(Debug)]
pub struct SomeService {
    _greet: Lifeline,
}

impl BridgeService for SomeService {}

impl Service for SomeService {
    type Bus = TemplateLinkedBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<TemplateLinkedMessage>()?;
        let _component_http_client = HttpClientComponent::restore::<TemplateLinked>()?;
        let _greet = Self::try_task(
            &format!("{}-service-some", TemplateLinked::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    log::debug!(
                        target: TemplateLinked::NAME,
                        "[{}] recv a new some message: {:?}",
                        TemplateLinked::NAME,
                        message
                    );
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
