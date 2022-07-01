use support_lifeline::task::TaskStack;

use crate::bridge::PangoroKilnBus;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PangoroKilnServiceManager {
    stack: TaskStack<PangoroKilnBus>,
}

impl PangoroKilnServiceManager {
    pub async fn new() -> color_eyre::Result<Self> {
        let bus = PangoroKilnBus::default();
        let mut stack = TaskStack::new(bus);
        Ok(Self { stack })
    }
}
