use crate::{
    error::Result,
    Settings,
	api::darwinia_api,
};
use parity_scale_codec::Encode;

use darwinia::{Darwinia2Ethereum, EventInfo, FormatedMMR};

use colored::*;
use std::fmt;
use rpassword::prompt_password_stdout;

#[derive(Default, Debug)]
struct TxProofWithMMRProof {
	message: String,
	signatures: Vec<String>,
	root: String,
	mmrindex: u64,
	block_header: String,
	peaks: Vec<String>,
	siblings: Vec<String>,
	events_proof_str: String,
	signers: Vec<String>,
}

impl fmt::Display for TxProofWithMMRProof {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			r#"{{
    message: {},
    signatures: [{}],
    root: {},
    MMRIndex: {},
    blockNumber: {},
    peaks: [{}],
    siblings: [{}],
    eventsProofStr: {},
    signers: {}
}}"#,
			self.message.green(),
			self.signatures.join(",").green(),
			self.root.green(),
			self.mmrindex.to_string().red(),
			self.block_header.green(),
			self.peaks.join(",").green(),
			self.siblings.join(",").green(),
			self.events_proof_str.green(),
			self.signers.join(","),
		)
	}
}

/// Get Darwinia to Ethereum Info
pub async fn exec(network: String, txblock: u64, mmrblock: u64, signblock: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?;
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let darwinia2ethereum = darwinia_api::get_d2e_instance(darwinia.clone());
	// mmr root block
	let mmr_root = darwinia.get_mmr_root(mmrblock as u32).await?;
	let message = Darwinia2Ethereum::construct_mmr_root_message(network, mmrblock as u32, mmr_root);
	//let message = web3::signing::keccak256(&encoded);

	let header = darwinia.get_block_by_number(txblock as u32).await?;
	let proof = darwinia2ethereum
		.get_headermmr_genproof(txblock, mmrblock, header.hash())
		.await?;
	let event_proof = darwinia
		.get_event_proof(
			hex::decode("f8860dda3d08046cf2706b92bf7202eaae7a79191c90e76297e0895605b8b457")
				.unwrap(),
			header.hash(),
		)
		.await?;

	//let mut result = HashMap::new();
	let mut result: TxProofWithMMRProof = Default::default();

	result.message = "0x".to_string() + &hex::encode(&message);
	result.block_header = "0x".to_string() + &hex::encode(header.encode());
	result.events_proof_str = "0x".to_string()
		+ &hex::encode(
			event_proof
				.iter()
				.map(|x| &x.0)
				.collect::<Vec<&Vec<u8>>>()
				.encode(),
		);
	result.root = "0x".to_string() + &hex::encode(mmr_root);
	result.mmrindex = mmrblock;

	if let Some(header_proof) = proof {
		let proof: Option<FormatedMMR> = header_proof.into();
		if let Some(formated_proof) = proof {
			result.peaks = formated_proof.peaks;
			result.siblings = formated_proof.siblings;
		}
	}

	// confirmed block
	let events = darwinia
		.get_events_from_block_number(signblock as u32)
		.await?;
	for event in events {
		if let EventInfo::MMRRootSignedEvent(signed_info) = event {
			let mut sign_accounts = vec![];
			let mut signatures = vec![];
			for signature in signed_info.signatures {
				sign_accounts.push(signature.0.to_string());
				signatures.push("0x".to_string() + &hex::encode(signature.1.encode()));
			}
			//result.insert("sign-account".to_string(), sign_accounts.join(","));
			result.signatures = signatures;
			result.signers = sign_accounts;
		}
	}
	println!("{}", result);
	Ok(())
}
