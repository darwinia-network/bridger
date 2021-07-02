use std::fmt::Debug;

use crate::bridge::service::BridgeService;

pub trait BridgeSand {
    const NAME: &'static str;
}

pub trait BridgeTask<B: lifeline::Bus>: BridgeSand + BridgeTaskManage<B> {
    fn spawn_service<
        S: lifeline::Service<Bus = B, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        bus: &B,
    ) -> anyhow::Result<Box<dyn BridgeService + Send + Sync>> {
        Ok(Box::new(S::spawn(bus)?))
    }
}

pub trait BridgeTaskManage<B: lifeline::Bus>: Debug {
    fn bus(&self) -> &B;
    fn keep_carry(&mut self, other_bus: lifeline::Lifeline);
}
