use support_lifeline::task::TaskStack;

use crate::bridge::PangolinRococoBus;

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
        Ok(Self { stack })
    }
}

impl PangolinRococoTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<PangolinRococoBus> {
        &self.stack
    }
}
