use lifeline::Message;
use postage::broadcast;

use crate::bridge::TemplateTaskBus;

#[derive(Debug, Clone)]
pub enum TemplateTaskMessage {
    SomeEvent(u64),
    StopSomeService,
}

impl Message<TemplateTaskBus> for TemplateTaskMessage {
    type Channel = broadcast::Sender<Self>;
}
