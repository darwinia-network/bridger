use support_lifeline::task::TaskStack;

use crate::bridge::PangoroChapelBus;
use crate::service::some::SomeService;

#[derive(Debug)]
pub struct PangoroChapelTask {
    stack: TaskStack<PangoroChapelBus>,
}

impl PangoroChapelTask {
    pub fn new() -> color_eyre::Result<Self> {
        let bus = PangoroChapelBus::default();
        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<SomeService>()?;
        Ok(Self { stack })
    }
}

impl PangoroChapelTask {
    pub fn stack(&self) -> &TaskStack<PangoroChapelBus> {
        &self.stack
    }
}
