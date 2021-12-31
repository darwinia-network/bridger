use support_lifeline::task::TaskStack;

use crate::bridge::DarwiniaCrabBus;
use crate::service::fee::UpdateFeeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct DarwiniaCrabTask {
    stack: TaskStack<DarwiniaCrabBus>,
}

impl DarwiniaCrabTask {
    pub fn name() -> &'static str {
        "task-darwinia-crab"
    }
}

impl DarwiniaCrabTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = DarwiniaCrabBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<RelayService>()?;
        stack.spawn_service::<UpdateFeeService>()?;

        Ok(Self { stack })
    }
}

impl DarwiniaCrabTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<DarwiniaCrabBus> {
        &self.stack
    }
}
