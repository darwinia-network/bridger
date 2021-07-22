use lifeline::Message;
use postage::broadcast;

use crate::bus::TemplateTaskBus;

#[derive(Debug, Clone)]
pub enum TemplateTaskMessage {
    SomeEvent,
    StopSomeService,
}

impl Message<TemplateTaskBus> for TemplateTaskMessage {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone)]
pub enum ToTemplateLinkedMessage {
    SomeEvent,
}

impl Message<TemplateTaskBus> for ToTemplateLinkedMessage {
    type Channel = broadcast::Sender<Self>;
}
