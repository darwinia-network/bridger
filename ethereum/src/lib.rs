mod error;
pub mod ethereum_api;
mod ethereum_tracker;
mod chains;

pub use error::{Error, Result};
pub use chains::{TrackContext, LogsHandler};
pub use chains::{EthereumLikeChain, DefaultLogsHandler, TopicsList, Ethereum, Heco, Bsc};
pub use ethereum_tracker::EthereumLikeChainTracker;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
