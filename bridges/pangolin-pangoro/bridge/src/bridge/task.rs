use lifeline::{Bus, Sender};

use support_lifeline::task::TaskStack;

use crate::bridge::PangolinPangoroBus;
use crate::bridge::PangolinPangoroMessageSend;
use crate::service::fee::UpdateFeeService;
use crate::service::relay::RelayService;

#[derive(Debug)]
pub struct PangolinPangoroTask {
    stack: TaskStack<PangolinPangoroBus>,
}

impl PangolinPangoroTask {
    pub fn name() -> &'static str {
        "task-pangolin-pangoro"
    }
}

impl PangolinPangoroTask {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangolinPangoroBus::default();

        let mut stack = TaskStack::new(bus);
        stack.spawn_service::<RelayService>()?;
        stack.spawn_service::<UpdateFeeService>()?;

        Ok(Self { stack })
    }
}

impl PangolinPangoroTask {
    #[allow(dead_code)]
    pub fn stack(&self) -> &TaskStack<PangolinPangoroBus> {
        &self.stack
    }
}
