use support_lifeline::task::TaskStack;

use crate::bridge::PangolinRococoBus;
use crate::service::header::HeaderRelayService;
use crate::service::message::MessageRelayService;

#[derive(Debug)]
pub struct PangolinRococoTask {
    stack: TaskStack<PangolinRococoBus>,
}

impl PangolinRococoTask {
    pub fn name() -> &'static str {
        "task-pangolin-rococo"
    }
}

impl PangolinRococoTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangolinRococoBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<PangolinToParachainHeaderRelayService>()?;
        // stack.spawn_service::<MessageRelayService>()?;
        Ok(Self { stack })
    }
}

impl PangolinRococoTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<PangolinRococoBus> {
        &self.stack
    }
}
