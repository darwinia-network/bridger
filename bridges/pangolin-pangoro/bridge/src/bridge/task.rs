use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::feemarket::FeemarketService;
use crate::service::header::pangolin_to_pangoro::PangolinToPangoroHeaderRelayService;
use crate::service::header::pangoro_to_pangolin::PangoroToPangolinHeaderRelayService;
use crate::service::message::pangolin_to_pangoro::PangolinToPangoroMessageRelayService;
use crate::service::message::pangoro_to_pangolin::PangoroToPangolinMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "pangolin-pangoro"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SubscribeService>()?;
        stack.spawn_service::<PangolinToPangoroHeaderRelayService>()?;
        stack.spawn_service::<PangoroToPangolinHeaderRelayService>()?;
        stack.spawn_service::<PangolinToPangoroMessageRelayService>()?;
        stack.spawn_service::<PangoroToPangolinMessageRelayService>()?;
        stack.spawn_service::<FeemarketService>()?;

        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
