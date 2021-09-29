#![allow(clippy::ptr_arg)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub use chains::{Bsc, Ethereum, Heco};
pub use error::Error;
pub use evm_client::EvmClient;
pub use traits::*;

pub use self::evm_log_tracker::EvmLogTracker;

mod chains;
mod error;
mod evm_client;
mod evm_log_tracker;
mod impls;
mod traits;

pub type Result<T> = anyhow::Result<T>;
