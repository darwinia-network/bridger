use crate::{
	api::Shadow,
	error::{BizError, Result},
	Config,
};
use primitives::chain::ethereum::EthereumHeader;

use darwinia::{Darwinia, DarwiniaAccount, Ethereum2Darwinia, FromEthereumAccount};

/// Affirm Force
pub async fn exec(block: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Config::new(&Config::default_data_dir()?)?; // TODO: add --data-dir
	let shadow = Shadow::new(&config);
	let darwinia = Darwinia::new(&config.node).await?;
	let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
	let darwinia_account = DarwiniaAccount::new(
		config.seed.clone(),
		config
			.proxy
			.clone()
			.map(|proxy| proxy.real[2..].to_string()),
	);
	let from_ethereum_account = FromEthereumAccount::new(darwinia_account);

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
