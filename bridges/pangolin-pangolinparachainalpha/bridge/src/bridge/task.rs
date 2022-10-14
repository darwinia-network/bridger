use support_lifeline::task::TaskStack;

use crate::bridge::BridgeBus;
use crate::service::header::{
    MoonbaseToPangolinHeaderRelayService, PangolinToParachainAlphaHeaderRelayService,
    ParaHeadRelayService,
};
use crate::service::message::pangolin_to_pangolinparachain::PangolinToPangolinParachainAlphaMessageRelayService;
use crate::service::message::pangolinparachain_to_pangolin::PangolinParachainAlphaToPangolinMessageRelayService;
use crate::service::subscribe::SubscribeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeBus>,
}

impl BridgeTask {
    pub fn name() -> &'static str {
        "task-pangolin-pangolinparachainalpha"
    }
}

impl BridgeTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = BridgeBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<PangolinToParachainAlphaHeaderRelayService>()?;
        stack.spawn_service::<MoonbaseToPangolinHeaderRelayService>()?;
        stack.spawn_service::<ParaHeadRelayService>()?;
        stack.spawn_service::<PangolinToPangolinParachainAlphaMessageRelayService>()?;
        stack.spawn_service::<PangolinParachainAlphaToPangolinMessageRelayService>()?;
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
