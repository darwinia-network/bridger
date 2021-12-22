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
