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
	/// secret_key to send ethereum tx
	pub secret_key: Option<SecretKey>,
	beneficiary: Option<String>,
}

impl Ethereum {
	/// new
	pub fn new(web3: Web3<Http>, config: &Config) -> Result<Self> {
		let relay_contract_address = Ethereum::build_address(&config.eth.contract.relay.address)?;

		let secret_key = if let Some(seed) = config.darwinia_to_ethereum.seed.clone() {
			let private_key = hex::decode(&seed[2..])?;
			Some(SecretKey::from_slice(&private_key)?)
		} else {
			None
		};

		Ok(Ethereum {
			web3,
			relay_contract_address,
			secret_key,
			beneficiary: config.darwinia_to_ethereum.beneficiary.clone(),
		})
	}

	/// submit_authorities_set
	pub async fn submit_authorities_set(
		&self,
		message: Vec<u8>,
		signatures: Vec<EcdsaSignature>,
	) -> Result<()> {
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
				for signature in signature_list.clone() {
					debug!("signature: 0x{}", hex::encode(signature));
				}
				debug!("beneficiary: 0x{}", hex::encode(beneficiary_buffer));

				let input = (message, signature_list, beneficiary_buffer);
				let receipt = contract
					.signed_call_with_confirmations(
						"updateRelayer",
						input,
						Options::with(|options| {
							options.gas = Some(150_000.into());
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
