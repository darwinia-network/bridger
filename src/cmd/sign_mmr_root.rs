use crate::{error::Result, Config};

use darwinia::{Darwinia, Darwinia2Ethereum, DarwiniaAccount, ToEthereumAccount};

/// Sign darwinia mmr root, the current block must be larger then mmrblock
pub async fn exec(network: String, mmrblock: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Config::new(&Config::default_data_dir()?)?;
	let darwinia = Darwinia::new(&config.node).await?;
	let darwinia_account = DarwiniaAccount::new(
		config.seed.clone(),
		config
			.proxy
			.clone()
			.map(|proxy| proxy.real[2..].to_string()),
	);
	let to_ethereum_account = ToEthereumAccount::new(
		darwinia_account,
		config.darwinia_to_ethereum.seed.clone(),
		config.eth.rpc.to_string(),
	);
	let darwinia2ethereum = Darwinia2Ethereum::new(darwinia.clone());

	let tx = darwinia2ethereum
		.ecdsa_sign_and_submit_signed_mmr_root(&to_ethereum_account, network, mmrblock as u32)
		.await?;

	println!("{}", tx);
	Ok(())
}
