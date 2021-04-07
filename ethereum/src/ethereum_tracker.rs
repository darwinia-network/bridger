use crate::Result;
use crate::{TrackContext, LogsHandler};
use crate::EthereumLikeChain;
use crate::ethereum_api::get_logs;

use web3::{
	transports::http::Http,
	types::Log,
	Web3,
};
use tokio::time::{delay_for, Duration};

pub struct EthereumLikeChainTracker<C: TrackContext, H: LogsHandler> {
	web3: Web3<Http>,
	chain: EthereumLikeChain<C, H>,
	stop: bool,
}

impl<C: TrackContext, H: LogsHandler> EthereumLikeChainTracker<C, H> {
	pub fn new(web3: Web3<Http>, chain: EthereumLikeChain<C, H>) -> Self {
		EthereumLikeChainTracker {
			web3,
			chain,
			stop: false,
		}
	}

	pub async fn start(&mut self) {
		loop {
			match self.next().await {
				Err(err) => {
					error!("{:?}", err);
					delay_for(Duration::from_secs(30)).await;
				},
				Ok(logs) => {
					if let Err(err2) = self.chain.handle(logs).await {
						error!("{:?}", err2);
					}
				}
			}

			if self.stop == true {
				break;
			}
		}
	}

	pub fn stop(&mut self) {
		self.stop = true;
	}

	pub async fn next(&mut self) -> Result<Vec<Log>> {
		let mut result = vec![];
		let (from, to) = self.chain.next_range(&self.web3).await?;
		info!(
			"Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
			self.chain.name(),
			from, to
		);
		for topics in &self.chain.get_topics_list() {
			let logs = get_logs(&self.web3, &topics.0, &topics.1, from, to).await?;
			result.extend_from_slice(&logs);
		}
		Ok(result)
	}

}

#[tokio::test]
async fn test_ethereum() {
	use crate::Ethereum;
	use crate::TopicsList;
	use crate::DefaultLogsHandler;
	use array_bytes::hex2bytes_unchecked as bytes;
	let web3 = Web3::new(Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap());

	let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
	let contract_address = H160::from_slice(&bytes(contract_address));

	let topics = &vec!["0x91d6d149c7e5354d1c671fe15a5a3332c47a38e15e8ac0339b24af3c1090690f"];
	let topics = topics
		.iter()
		.map(|t| H256::from_slice(&bytes(t)))
		.collect();

	let topics_list = TopicsList::new(
		vec![(contract_address, topics)],
		DefaultLogsHandler {},
	);

	let chain = EthereumLikeChain::new("Ethereum", topics_list, Ethereum::new(100));

	let mut tracker = EthereumLikeChainTracker::new(
		web3.clone(), 
		chain,
	);

	tracker.start();
}
