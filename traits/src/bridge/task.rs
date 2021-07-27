use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::bridge::service::BridgeService;

pub trait BridgeSand {
    const NAME: &'static str;
}

pub trait BridgeTask<B: lifeline::Bus>: BridgeSand + BridgeTaskKeep {
    fn config_template() -> anyhow::Result<serde_json::Value>;
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
    bus: B,
}

impl<B: lifeline::Bus> TaskStack<B> {
    pub fn new(bus: B) -> Self {
        Self {
            services: Default::default(),
            carries: Default::default(),
            bus,
        }
    }
}

impl<B: lifeline::Bus> TaskStack<B> {
    pub fn bus(&self) -> &B {
        &self.bus
    }
    pub fn spawn_service<
        S: lifeline::Service<Bus = B, Lifeline = anyhow::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        &mut self,
    ) -> anyhow::Result<()> {
        let type_name = std::any::type_name::<S>();
        let service = Box::new(S::spawn(&self.bus)?);
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
    ) -> anyhow::Result<()> {
        // keep it until leave this block
        let _ = self.stop_service::<S>();
        self.spawn_service::<S>()
    }

    pub fn carry_from<CY: lifeline::Bus>(&mut self, other: &TaskStack<CY>) -> anyhow::Result<()>
    where
        B: lifeline::Bus
            + lifeline::prelude::CarryFrom<CY, Lifeline = anyhow::Result<lifeline::Lifeline>>,
    {
        let lifeline = self.bus.carry_from(&other.bus)?;
        self.carries.push(lifeline);
        Ok(())
    }

    // pub fn carry(&mut self, lifeline: lifeline::Lifeline) -> anyhow::Result<()> {
    //     self.carries.push(lifeline);
    //     Ok(())
    // }
}
