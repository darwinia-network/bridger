use crate::Result;
use crate::LogsHandler;
use web3::types::Log;
use tokio::time::{delay_for, Duration};
use crate::client::EthereumLikeChainClient;
use crate::chains::Chain;
use web3::types::{H160, H256};
use std::marker::PhantomData;

pub struct EthereumLikeChainTracker<C: Chain, H: LogsHandler> {
	client: EthereumLikeChainClient,
	topics_list: Vec<(H160, Vec<H256>)>,
	logs_handler: H,
	from: u64,
	stop: bool,
	phantom: PhantomData<C>,
}

impl<C: Chain, H: LogsHandler> EthereumLikeChainTracker<C, H> {
	pub fn new(client: EthereumLikeChainClient, topics_list: Vec<(H160, Vec<H256>)>, logs_handler: H, from: u64) -> Self {
		EthereumLikeChainTracker {
			client,
			topics_list,
			logs_handler,
			from,
			stop: false,
			phantom: PhantomData,
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
					if let Err(err2) = self.handle(logs).await {
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
		let (from, to) = self.next_range().await?;
		info!(
			"Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
			C::NAME,
			from, to
		);
		for topics in &self.topics_list {
			let logs = self.client.get_logs(&topics.0, &topics.1, from, to).await?;
			result.extend_from_slice(&logs);
		}
		Ok(result)
	}

	async fn next_range(&mut self) -> Result<(u64, u64)> {
		let range = C::next_range(self.from, &self.client).await?;
		self.from = range.1;
		Ok(range)
	}

	async fn handle(&self, logs: Vec<Log>) -> Result<()> {
		self.logs_handler.handle(&self.client, &self.topics_list, logs).await?;
		Ok(())
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
