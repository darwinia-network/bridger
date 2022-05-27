use support_lifeline::task::TaskStack;

use crate::bridge::BridgeTaskBus;
use crate::service::feemarket::FeemarketService;
use crate::service::header::pangolin_to_pangoro::PangolinToPangoroHeaderRelayService;
use crate::service::header::pangoro_to_pangolin::PangoroToPangolinHeaderRelayService;
use crate::service::message::MessageRelayService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeTaskBus>,
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeTaskBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<PangolinToPangoroHeaderRelayService>()?;
        stack.spawn_service::<PangoroToPangolinHeaderRelayService>()?;
        stack.spawn_service::<MessageRelayService>()?;
        stack.spawn_service::<FeemarketService>()?;

        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeTaskBus> {
        &self.stack
    }
}
