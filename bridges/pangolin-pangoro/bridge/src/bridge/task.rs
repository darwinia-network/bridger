use support_lifeline::task::TaskStack;

use crate::bridge::BridgeTaskBus;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct BridgeTask {
    stack: TaskStack<BridgeTaskBus>,
}

impl BridgeTask {
    pub fn new() -> color_eyre::Result<Self> {
        let bus = BridgeTaskBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}

impl BridgeTask {
    pub fn stack(&self) -> &TaskStack<BridgeTaskBus> {
        &self.stack
    }
}
