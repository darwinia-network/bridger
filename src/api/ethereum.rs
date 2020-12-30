//! Ethereum API
use crate::{error::Result, Config};

use primitives::runtime::EcdsaSignature;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{Address, H160};
use web3::Web3;

/// Ethereum
pub struct Ethereum {
	web3: Web3<Http>,
	relay_contract_address: Address,
	secret_key: SecretKey,
	benefit: Option<String>,
}

impl Ethereum {
	/// new
	pub fn new(web3: Web3<Http>, config: &Config) -> Result<Self> {
		let relay_contract_address =
			Ethereum::build_address(&config.darwinia_to_ethereum.relay_contract_address)?;
		let private_key = hex::decode(&config.darwinia_to_ethereum.seed[2..])?;
		let secret_key = SecretKey::from_slice(&private_key)?;
		Ok(Ethereum {
			web3,
			relay_contract_address,
			secret_key,
			benefit: config.darwinia_to_ethereum.benefit.clone(),
		})
	}

	/// submit_authorities_set
	pub async fn submit_authorities_set(
		&self,
		message: Vec<u8>,
		signatures: Vec<EcdsaSignature>,
	) -> Result<()> {
		if let Some(benefit) = &self.benefit {
			let key_ref = SecretKeyRef::new(&self.secret_key);

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

			// benefit account id
			let benefit = hex::decode(&benefit[2..])?;
			let mut benefit_buffer = [0u8; 32];
			benefit_buffer.copy_from_slice(&benefit);

			// debug
			debug!("message: 0x{}", hex::encode(message.clone()));
			for signature in signature_list.clone() {
				debug!("signature: 0x{}", hex::encode(signature));
			}
			debug!("benefit: 0x{}", hex::encode(benefit_buffer));

			let input = (message, signature_list, benefit_buffer);
			let receipt = contract
				.signed_call_with_confirmations(
					"updateRelayer",
					input,
					Options::with(|options| {
						options.gas = Some(500_000.into());
					}),
					12,
					key_ref,
				)
				.await?;
			trace!(
				"Submit authorities to eth with tx: {}, status: {:?}",
				receipt.transaction_hash,
				receipt.status
			);
		}

		Ok(())
	}

	fn build_address(str: &str) -> Result<H160> {
		let address = hex::decode(&str[2..])?;
		let mut address_buffer = [0u8; 20];
		address_buffer.copy_from_slice(&address);
		Ok(Address::from(address_buffer))
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
