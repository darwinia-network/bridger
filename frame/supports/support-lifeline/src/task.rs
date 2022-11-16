use std::collections::HashMap;

use crate::service::BridgeService;

/// Lifeline task stack, keep all running services
#[derive(Debug, Default)]
pub struct TaskStack<B: lifeline::Bus> {
    services: HashMap<String, Box<dyn BridgeService + Send + Sync>>,
    carries: Vec<lifeline::Lifeline>,
    bus: B,
}

impl<B: lifeline::Bus> TaskStack<B> {
    /// Create a task stack
    pub fn new(bus: B) -> Self {
        Self {
            services: Default::default(),
            carries: Default::default(),
            bus,
        }
    }
}

impl<B: lifeline::Bus> TaskStack<B> {
    /// Get bus reference
    pub fn bus(&self) -> &B {
        &self.bus
    }

    /// Spawn lifeline service
    pub fn spawn_service<
        S: lifeline::Service<Bus = B, Lifeline = color_eyre::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        &mut self,
    ) -> color_eyre::Result<()> {
        let type_name = std::any::type_name::<S>();
        let service = Box::new(S::spawn(&self.bus)?);
        self.services.insert(type_name.to_string(), service);
        Ok(())
    }

    /// Stop lifeline service
    pub fn stop_service<
        S: lifeline::Service<Bus = B, Lifeline = color_eyre::Result<S>> + BridgeService,
    >(
        &mut self,
    ) -> Option<Box<dyn BridgeService + Send + Sync>> {
        let type_name = std::any::type_name::<S>();
        self.services.remove(type_name)
    }

    /// Respawn lifeline service
    pub fn respawn_service<
        S: lifeline::Service<Bus = B, Lifeline = color_eyre::Result<S>>
            + BridgeService
            + Send
            + Sync
            + 'static,
    >(
        &mut self,
    ) -> color_eyre::Result<()> {
        // keep it until leave this block
        let _ = self.stop_service::<S>();
        self.spawn_service::<S>()
    }

    /// Lifeline service carry
    pub fn carry_from<CY: lifeline::Bus>(&mut self, other: &TaskStack<CY>) -> color_eyre::Result<()>
    where
        B: lifeline::Bus
            + lifeline::prelude::CarryFrom<CY, Lifeline = color_eyre::Result<lifeline::Lifeline>>,
    {
        let lifeline = self.bus.carry_from(&other.bus)?;
        self.carries.push(lifeline);
        Ok(())
    }
}
