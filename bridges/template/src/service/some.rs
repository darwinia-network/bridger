use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use component_http_client::HttpClientComponent;
use support_common::config::{Config, Names};
use support_lifeline::service::BridgeService;

use crate::bridge::{TemplateTaskBus, TemplateTaskConfig, TemplateTaskMessage};

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
        let config: TemplateTaskConfig = Config::restore(Names::BridgeTemplate)?;
        let client = HttpClientComponent::component(config.http_client)?;

        let _greet = Self::try_task("template-service-some", async move {
            while let Some(message) = rx.recv().await {
                match message {
                    TemplateTaskMessage::SomeEvent(times) => {
                        let url = "https://httpbin.org/get";
                        let response = client.get(url).send().await?;
                        let body = response.text().await?;
                        tracing::debug!("Receive a new some event. times: {}.", times);
                        tracing::debug!("Try request {} and response is: {}", url, body);
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
