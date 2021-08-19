#![allow(clippy::ptr_arg)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub use self::evm_log_tracker::EvmLogTracker;
pub use chains::{Bsc, Ethereum, Heco};
pub use default_logs_handler::DefaultLogsHandler;
pub use error::Error;
pub use evm_client::EvmClient;
pub use traits::{EvmChain, LogsHandler};

mod chains;
mod default_logs_handler;
mod error;
mod evm_client;
mod evm_log_tracker;
mod traits;

pub type Result<T> = anyhow::Result<T>;
