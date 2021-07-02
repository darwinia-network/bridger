use lifeline::Message;
use postage::broadcast;

use crate::bus::TemplateTaskBus;

#[derive(Debug, Clone)]
pub enum TemplateTaskMessage {
    SomeEvent,
}

impl Message<TemplateTaskBus> for TemplateTaskMessage {
    type Channel = broadcast::Sender<Self>;
}

#[derive(Debug, Clone)]
pub enum ToTemplateLinkMessage {
    SomeEvent,
}

impl Message<TemplateTaskBus> for ToTemplateLinkMessage {
    type Channel = broadcast::Sender<Self>;
}
