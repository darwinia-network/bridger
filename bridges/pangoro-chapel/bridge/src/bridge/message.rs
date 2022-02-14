use lifeline::Message;
use postage::broadcast;

use crate::bridge::PangoroChapelBus;

#[derive(Debug, Clone)]
pub enum TemplateTaskMessage {
    SomeEvent(u64),
    StopSomeService,
}

impl Message<PangoroChapelBus> for TemplateTaskMessage {
    type Channel = broadcast::Sender<Self>;
}
