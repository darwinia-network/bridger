use crate::Result;
use crate::Chain;
use crate::ethereum_api::get_logs;

use web3::{
	transports::http::Http,
	types::{Log, H160, H256},
	Web3,
};

pub struct EthereumTracker<T: Chain> {
	web3: Web3<Http>,
	subscribes: Vec<(H160, Vec<H256>)>, // array of (contract_address, topics)
	pub chain: T,
}

impl<T: Chain> EthereumTracker<T> {
	pub fn new(web3: Web3<Http>, subscribes: Vec<(H160, Vec<H256>)>, chain: T) -> Self {
		EthereumTracker {
			web3,
			subscribes,
			chain
		}
	}

	pub async fn next(&mut self) -> Result<Vec<Log>> {
		let mut result = vec![];
		let (from, to) = self.chain.next_range().await?;
		info!(
			"Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
			self.chain.name(),
			from, to
		);
		for subscribe in self.subscribes.clone() {
			let logs = get_logs(&self.web3, subscribe.0, subscribe.1, from, to).await?;
			result.extend_from_slice(&logs);
		}
		Ok(result)
	}

}

#[tokio::test]
async fn test_ethereum() {
	use crate::Ethereum;
	use array_bytes::hex2bytes_unchecked as bytes;
	let web3 = Web3::new(Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap());

	let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
	let contract_address = H160::from_slice(&bytes(contract_address));

	let topics = &vec!["0x91d6d149c7e5354d1c671fe15a5a3332c47a38e15e8ac0339b24af3c1090690f"];
	let topics = topics
		.iter()
		.map(|t| H256::from_slice(&bytes(t)))
		.collect();

	let mut tracker = EthereumTracker::new(
		web3.clone(), 
		vec![(contract_address, topics)], 
		Ethereum { web3, from: 1 }
	);

	loop {
		let logs = tracker.next().await.unwrap();
		println!("logs: {}", logs.len());
	}
}

#[tokio::test]
async fn test_heco() {
	use crate::Heco;
	use array_bytes::hex2bytes_unchecked as bytes;
	let web3 = Web3::new(Http::new("https://http-mainnet-node.huobichain.com/").unwrap());

	let contract_address = "0xA929022c9107643515F5c777cE9a910F0D1e490C";
	let contract_address = H160::from_slice(&bytes(contract_address));

	let topics = &vec!["0x63333d9ba80d323ed3a2c486809215e8f1fb8c645691606b862787fad572c1c7"];
	let topics = topics
		.iter()
		.map(|t| H256::from_slice(&bytes(t)))
		.collect();

	let mut tracker = EthereumTracker::new(
		web3.clone(), 
		vec![(contract_address, topics)], 
		Heco { web3, from: 3445971, step: 5000 }
	);

	loop {
		let logs = tracker.next().await.unwrap();
		println!("logs: {}", logs.len());
	}
}


