use std::fmt::Debug;

use crate::config::BridgeConfig;

pub trait BridgeComponent<C: BridgeConfig, R> {
	fn component(&self) -> anyhow::Result<R>;
	fn config(&self) -> &C;
}
