use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::bridge::service::BridgeService;

pub trait BridgeSand {
    const NAME: &'static str;
}

pub trait BridgeTask<B: lifeline::Bus>: BridgeSand + BridgeTaskKeep {
    fn config_template() -> anyhow::Result<serde_json::Value>;

    fn bus(&self) -> &B;
    fn stack(&mut self) -> &mut TaskStack<B>;
}

#[async_trait::async_trait]
pub trait BridgeTaskKeep: Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

#[derive(Debug, Default)]
pub struct TaskStack<B: lifeline::Bus> {
    services: HashMap<String, Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
    _marker: PhantomData<fn() -> B>,
}

impl<B: lifeline::Bus> TaskStack<B> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<B: lifeline::Bus> TaskStack<B> {
    pub fn spawn_service<
        S: lifeline::Service<Bus = B, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        &mut self,
        bus: &B,
    ) -> anyhow::Result<()> {
        let type_name = std::any::type_name::<S>();
        let service = Box::new(S::spawn(bus)?);
        self.services.insert(type_name.to_string(), service);
        Ok(())
    }

    pub fn stop_service<
        S: lifeline::Service<Bus = B, Lifeline = anyhow::Result<S>> + BridgeService,
    >(
        &mut self,
    ) -> Option<Box<dyn BridgeService + Send + Sync>> {
        let type_name = std::any::type_name::<S>();
        self.services.remove(type_name)
    }

    pub fn respawn_service<
        S: lifeline::Service<Bus = B, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        &mut self,
        bus: &B,
    ) -> anyhow::Result<()> {
        // keep it until leave this block
        let _ = self.stop_service::<S>();
        self.spawn_service::<S>(bus)
    }

    pub fn carry(&mut self, lifeline: lifeline::Lifeline) -> anyhow::Result<()> {
        self.carries.push(lifeline);
        Ok(())
    }
}
