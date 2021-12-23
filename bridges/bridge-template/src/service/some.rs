use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use support_lifeline::service::BridgeService;

use crate::bridge::{TemplateTaskBus, TemplateTaskMessage};

#[derive(Debug)]
pub struct SomeService {
    _greet: Lifeline,
}

impl BridgeService for SomeService {}

impl Service for SomeService {
    type Bus = TemplateTaskBus;
    type Lifeline = color_eyre::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        tracing::trace!("Spawn service some");
        let mut rx = bus.rx::<TemplateTaskMessage>()?;
        let _greet = Self::try_task("template-service-some", async move {
            while let Some(message) = rx.recv().await {
                match message {
                    TemplateTaskMessage::SomeEvent => {
                        tracing::debug!("recv a new some message: {:?}", message);
                    }
                    TemplateTaskMessage::StopSomeService => {
                        break;
                    }
                }
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}
