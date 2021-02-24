use crate::{api::darwinia_api, error::Result, Settings};

use rpassword::prompt_password_stdout;

/// Sign darwinia mmr root, the current block must be larger then mmrblock
pub async fn exec(network: String, mmrblock: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?;
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let to_ethereum_account = darwinia_api::get_d2e_account(darwinia_account, &config);
	let darwinia2ethereum = darwinia_api::get_d2e_instance(darwinia);

	let tx = darwinia2ethereum
		.ecdsa_sign_and_submit_signed_mmr_root(&to_ethereum_account, network, mmrblock as u32)
		.await?;

	println!("{}", tx);
	Ok(())
}
