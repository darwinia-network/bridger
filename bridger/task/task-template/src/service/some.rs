use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};

use bridge_traits::bridge::component::BridgeComponent;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_http_client::HttpClientComponent;

use crate::bus::TemplateTaskBus;
use crate::message::{TemplateTaskMessage, ToTemplateLinkedMessage};
use crate::task::TemplateTask;

#[derive(Debug)]
pub struct SomeService {
    _greet: Lifeline,
}

impl BridgeService for SomeService {}

impl Service for SomeService {
    type Bus = TemplateTaskBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<TemplateTaskMessage>()?;
        let mut tx = bus.tx::<ToTemplateLinkedMessage>()?;
        let _component_http_client = HttpClientComponent::restore::<TemplateTask>()?;
        let _greet = Self::try_task(
            &format!("{}-service-some", TemplateTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    tx.send(ToTemplateLinkedMessage::SomeEvent).await?;
                    log::debug!(
                        target: TemplateTask::NAME,
                        "[{}] recv a new some message: {:?}",
                        TemplateTask::NAME,
                        message
                    );
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
