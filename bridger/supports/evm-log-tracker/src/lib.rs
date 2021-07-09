#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

mod chains;
mod default_logs_handler;
mod error;
mod evm_client;
mod evm_log_tracker;
mod traits;

pub use error::Error;
pub type Result<T> = anyhow::Result<T>;

pub use chains::{Bsc, Ethereum, Heco};
pub use default_logs_handler::DefaultLogsHandler;
pub use evm_client::EvmClient;
pub use evm_log_tracker::EvmLogTracker;
pub use traits::{EvmChain, LogsHandler};
