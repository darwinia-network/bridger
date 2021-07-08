use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

// Pin<Box<dyn Future<Output = T> + Send>>
use futures::future::BoxFuture;

use crate::bridge::service::BridgeService;

// todo: Fn(String) <--- the argument need change to real types
pub type TaskRouterAsyncFn =
    Box<dyn Send + Sync + Fn(String) -> BoxFuture<'static, anyhow::Result<serde_json::Value>>>;

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
    fn register_route(&self, router: &mut TaskRouter);
}

pub trait BridgeTaskKeep: Debug {
    fn as_any(&self) -> &dyn Any;
}

pub struct TaskRouter {
    router: HashMap<String, TaskRouterAsyncFn>,
}

impl TaskRouter {
    pub fn new() -> Self {
        Self {
            router: HashMap::new(),
        }
    }
}

impl TaskRouter {
    pub fn register<T: BridgeSand>(&mut self, uri: &str, func: TaskRouterAsyncFn) -> &mut Self {
        let key = format!("{}-{}", T::NAME, uri);
        self.router.insert(key, func);
        self
    }

    pub fn router(self) -> HashMap<String, TaskRouterAsyncFn> {
        self.router
    }
}
