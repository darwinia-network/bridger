use lifeline::Message;
use postage::broadcast;

use crate::bus::TemplateLinkedBus;

#[derive(Debug, Clone)]
pub enum TemplateLinkedMessage {
    SomeEvent,
}

impl Message<TemplateLinkedBus> for TemplateLinkedMessage {
    type Channel = broadcast::Sender<Self>;
}
