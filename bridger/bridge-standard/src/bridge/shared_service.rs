use std::fmt::Debug;

use crate::bridge::service::BridgeService;
use crate::bridge::task::BridgeTask;

pub trait SharedService<T: BridgeTask + 'static>: BridgeService<T> + Debug {
    fn spawn_with_shared(bus: &T::Bus) -> anyhow::Result<Self>;
}
