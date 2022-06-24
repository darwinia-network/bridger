use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::header::{
    PangolinToParachainHeaderRelayService, ParaHeadRelayService, RococoToPangolinHeaderRelayService,
};
use crate::service::message::pangolin_to_pangolinparachain::PangolinToPangolinParachainMessageRelayService;
use crate::service::message::pangolinparachain_to_pangolin::PangolinParachainToPangolinMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "task-pangolin-pangolinparachain"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<PangolinToParachainHeaderRelayService>()?;
        stack.spawn_service::<RococoToPangolinHeaderRelayService>()?;
        stack.spawn_service::<ParaHeadRelayService>()?;
        stack.spawn_service::<PangolinToPangolinParachainMessageRelayService>()?;
        stack.spawn_service::<PangolinParachainToPangolinMessageRelayService>()?;
        stack.spawn_service::<SubscribeService>()?;
        Ok(Self { stack })
    }
}

impl BridgeTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<BridgeBus> {
        &self.stack
    }
}
