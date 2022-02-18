use support_lifeline::task::TaskStack;

use crate::bridge::PangoroChapelBus;
use crate::service::header_relay::HeaderRelayService;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PangoroChapelServiceManager {
    stack: TaskStack<PangoroChapelBus>,
}

impl PangoroChapelServiceManager {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangoroChapelBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<HeaderRelayService>()?;
        Ok(Self { stack })
    }
}
