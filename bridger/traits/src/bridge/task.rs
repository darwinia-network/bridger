use std::any::Any;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::bridge::service::BridgeService;

pub trait BridgeSand {
    const NAME: &'static str;
}

pub trait BridgeTask<B: lifeline::Bus>: BridgeSand + BridgeTaskKeep {
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

    fn bus(&self) -> &B;
    fn keep_carry(&mut self, other_bus: lifeline::Lifeline);
}

#[async_trait::async_trait]
pub trait BridgeTaskKeep: Debug {
    fn as_any(&self) -> &dyn Any;
    async fn route(&self, uri: String, param: serde_json::Value) -> anyhow::Result<TaskTerminal>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskTerminal {
    view: String,
}

impl TaskTerminal {
    pub fn new(view: impl AsRef<str>) -> Self {
        Self {
            view: view.as_ref().to_string(),
        }
    }

    pub fn view(&self) -> &String {
        &self.view
    }
}
