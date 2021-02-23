//! Ethereum API
use crate::{error::Result, Settings};

use primitives::runtime::EcdsaSignature;
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{Address, H160, H256, U256};
use web3::Web3;

#[derive(Debug, Serialize, Deserialize)]
struct GasPrice {
	code: i32,
	data: GasPriceData,
}

#[derive(Debug, Serialize, Deserialize)]
struct GasPriceData {
	rapid: u64,
	fast: u64,
	slow: u64,
	standard: u64,
	timestamp: u64,
}

/// Ethereum
pub struct Ethereum {
	web3: Web3<Http>,
	relay_contract_address: Address,
	/// secret_key to send ethereum tx
	pub secret_key: Option<SecretKey>,
	beneficiary: Option<String>,
}

impl Ethereum {
	/// new
	pub fn new(web3: Web3<Http>, config: &Settings) -> Result<Self> {
		let relay_contract_address =
			Ethereum::build_address(&config.ethereum.contract.relay.address)?;

		let secret_key = if let Some(seed) = config.ethereum.relayer.clone().map(|r| r.private_key)
		{
			let private_key = hex::decode(&seed[2..])?;
			Some(SecretKey::from_slice(&private_key)?)
		} else {
			None
		};

		Ok(Ethereum {
			web3,
			relay_contract_address,
			secret_key,
			beneficiary: config
				.ethereum
				.relayer
				.clone()
				.map(|r| r.beneficiary_darwinia_account),
		})
	}

	/// new2
	pub fn new2(
		web3: Web3<Http>,
		relay_contract_address: String,
		seed: Option<String>,
		beneficiary: Option<String>,
	) -> Result<Self> {
		let relay_contract_address = Ethereum::build_address(&relay_contract_address)?;

		let secret_key = if let Some(seed) = seed {
			let private_key = hex::decode(&seed[2..])?;
			Some(SecretKey::from_slice(&private_key)?)
		} else {
			None
		};

		Ok(Ethereum {
			web3,
			relay_contract_address,
			secret_key,
			beneficiary,
		})
	}

	/// is relayer
	pub fn is_relayer(&self) -> bool {
		self.beneficiary != None
	}

	/// submit_authorities_set
	pub async fn submit_authorities_set(
		&self,
		message: Vec<u8>,
		signatures: Vec<EcdsaSignature>,
	) -> Result<H256> {
		if let Some(beneficiary) = &self.beneficiary {
			if let Some(secret_key) = &self.secret_key {
				let key_ref = SecretKeyRef::new(secret_key);

				let contract = Contract::from_json(
					self.web3.eth(),
					self.relay_contract_address,
					include_bytes!("Relay.json"),
				)?;

				// signatures
				let signature_list = signatures
					.iter()
					.map(|item| item.0.to_vec())
					.collect::<Vec<_>>();

				// beneficiary account id
				let beneficiary = hex::decode(&beneficiary[2..])?;
				let mut beneficiary_buffer = [0u8; 32];
				beneficiary_buffer.copy_from_slice(&beneficiary);

				// debug
				debug!("message: 0x{}", hex::encode(message.clone()));
				for (i, signature) in signature_list.clone().iter().enumerate() {
					debug!("signature {}: 0x{}", i + 1, hex::encode(signature));
				}
				debug!("beneficiary: 0x{}", hex::encode(beneficiary_buffer));

				// gas price
				// TODO: do not need to get gas_price if ropsten
				let gas_price = Ethereum::fast_gas_price().await.ok();

				let input = (message, signature_list, beneficiary_buffer);
				let txhash = contract
					.signed_call(
						"updateRelayer",
						input,
						Options::with(|options| {
							options.gas = Some(150_000.into());
							options.gas_price = gas_price;
						}),
						key_ref,
					)
					.await?;
				Ok(txhash)
			} else {
				anyhow::bail!("You have no ethereum private key configured.")
			}
		} else {
			anyhow::bail!("You have no beneficiary configured.")
		}
	}

	fn build_address(str: &str) -> Result<H160> {
		let address = hex::decode(&str[2..])?;
		let mut address_buffer = [0u8; 20];
		address_buffer.copy_from_slice(&address);
		Ok(Address::from(address_buffer))
	}

	async fn fast_gas_price() -> Result<U256> {
		let gasnow_url = "https://gasnow.sparkpool.com/api/v3/gas/price?utm_source=DarwiniaBridger";
		let client = reqwest::Client::builder()
			.timeout(Duration::from_secs(15))
			.build()?;
		let gas_price: GasPrice = client.get(gasnow_url).send().await?.json().await?;
		Ok(gas_price.data.fast.into())
	}
}

#[test]
fn test_load_abi() {
	let web3 = Web3::new(
		Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
	);
	let relay_contract_address =
		Ethereum::build_address("0xeb931f1a197f3a230d2dfe220ac6674880f827d6").unwrap();
	let contract = Contract::from_json(
		web3.eth(),
		relay_contract_address,
		include_bytes!("Relay.json"),
	)
	.unwrap();
	println!("{:?}", contract);
}

#[actix_rt::test]
async fn test_submit_authorities_set() {
	let web3 = Web3::new(
		Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
	);

	let ethereum = Ethereum::new2(
		web3,
		"0xc8d6c331030886716f6e323ACB795077eB530E36".to_string(),
		Some("0x2f460a73df143256460ebb319b171a4024db8ce2d42bb83a42930814a50d2b71".to_string()),
		Some("0x129f002b1c0787ea72c31b2dc986e66911fe1b4d6dc16f83a1127f33e5a74c7d".to_string()),
	)
	.unwrap();

	let txhash = ethereum
		.submit_authorities_set(vec![0u8], vec![EcdsaSignature::default()])
		.await
		.unwrap();

	println!("{:?}", txhash);
}

#[actix_rt::test]
async fn test_calc_gas_price() {
	println!("{}", Ethereum::fast_gas_price().await.unwrap());
}
