use crate::{
	api::{darwinia_api, Shadow},
	error::{BizError, Result},
	Settings,
};
use primitives::chain::ethereum::EthereumHeader;
use rpassword::prompt_password_stdout;

/// Affirm Force
pub async fn exec(block: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let mut config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let shadow = Shadow::new(&config);
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia.clone());
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let from_ethereum_account = darwinia_api::get_e2d_account(darwinia_account);

	let parcel = shadow.parcel(block as usize + 1).await?;
	let block_number = parcel.header.number;
	if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
		return Err(BizError::ParcelFromShadowIsEmpty(block).into());
	}
	let ex_hash = ethereum2darwinia
		.affirm(&from_ethereum_account, parcel)
		.await?;
	info!(
		"Affirmed ethereum block {} in extrinsic {:?}",
		block_number, ex_hash
	);

	Ok(())
}
