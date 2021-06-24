use std::fmt::Debug;

use crate::bridge::task::BridgeTask;

pub trait BridgeService<T: BridgeTask + 'static>: Debug {}
