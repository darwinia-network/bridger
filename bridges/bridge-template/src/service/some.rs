use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};

use component_http_client::HttpClientComponent;
use support_lifeline::service::BridgeService;

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
                    match message {
                        TemplateTaskMessage::SomeEvent => {
                            tx.send(ToTemplateLinkedMessage::SomeEvent).await?;
                            tracing::debug!(
                                "[{}] recv a new some message: {:?}",
                                TemplateTask::NAME,
                                message
                            );
                        }
                        TemplateTaskMessage::StopSomeService => {
                            let task: &mut TemplateTask =
                                support_keep::task::running_task_downcast_mut(TemplateTask::NAME)?;
                            let stack = task.stack();
                            stack.stop_service::<SomeService>().expect("unreachable");
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
