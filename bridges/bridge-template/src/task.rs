use lifeline::dyn_bus::DynBus;

use support_lifeline::task::TaskStack;

use crate::bus::TemplateTaskBus;
use crate::config::TemplateTaskConfig;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct TemplateTask {
    stack: TaskStack<TemplateTaskBus>,
}

impl BridgeTask<TemplateTaskBus> for TemplateTask {
    fn stack(&mut self) -> &mut TaskStack<TemplateTaskBus> {
        &mut self.stack
    }
}

impl TemplateTask {
    pub fn new(config: TemplateTaskConfig) -> color_eyre::Result<Self> {
        let bus = TemplateTaskBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}
