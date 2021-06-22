use bridge_standard::bridge::chain::BridgeChain;
use lifeline::dyn_bus::DynBus;
use std::fmt::Debug;

pub struct Task {
    source: Option<dyn BridgeChain>,
    target: Option<dyn BridgeChain>,
    name: Option<String>,
    bus: Option<dyn DynBus + Debug + Clone>,
}

impl Task {
    pub fn with<B: DynBus + Debug + Clone>(bus: B) -> Self {
        Self {
            source: None,
            target: None,
            name: None,
            bus: Some(bus),
        }
    }
}

impl Task {
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    pub fn source<T: BridgeChain>(&mut self, source: T) -> &mut Self {
        self.source = Some(source);
        self
    }

    pub fn target<T: BridgeChain>(&mut self, target: T) -> &mut Self {
        self.target = Some(target);
        self
    }
}
