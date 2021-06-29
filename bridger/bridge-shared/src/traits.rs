use std::fmt::Debug;

use lifeline::dyn_bus::DynBus;
use lifeline::{Bus, Task};

use bridge_standard::bridge::chain::BridgeChain;
use bridge_standard::bridge::sand::BridgeSand;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeTask;

use crate::channel::SharedChannel;

pub trait SharedKeepService: Debug {}

pub trait SharedMaterial: BridgeSand + Clone + Debug {
    type Chain: BridgeChain;
    type Bus: DynBus + Default;

    fn bus() -> Self::Bus {
        Self::Bus::default()
    }
}

pub trait SharedService: Task + BridgeService + Debug {
    /// The bus, which must be provided to spawn the task
    type Bus: Bus;

    /// The service lifeline.  When dropped, all spawned tasks are immediately cancelled.
    type Lifeline;

    /// Spawns the service with all sub-tasks, and returns a lifeline value.  When the lifeline is dropped, all spawned tasks are immediately cancelled.
    ///
    /// Implementations should synchronously take channels from the bus, and then use them asynchronously.  This makes errors occur as early and predictably as possible.
    fn spawn(bus: &Self::Bus, channel: SharedChannel) -> Self::Lifeline;
}
