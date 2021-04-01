mod error;
pub mod ethereum_api;
mod ethereum_tracker;
mod chains;

pub use error::{Error, Result};
pub use chains::{Chain, Ethereum, Heco, Bsc};
pub use ethereum_tracker::EthereumTracker;

#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
