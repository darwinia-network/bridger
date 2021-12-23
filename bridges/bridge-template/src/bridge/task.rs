use support_lifeline::task::TaskStack;

use crate::bridge::TemplateTaskBus;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateTask {
    stack: TaskStack<TemplateTaskBus>,
}

impl TemplateTask {
    pub fn new() -> color_eyre::Result<Self> {
        let bus = TemplateTaskBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}

impl TemplateTask {
    pub fn stack(&self) -> &TaskStack<TemplateTaskBus> {
        &self.stack
    }
}
