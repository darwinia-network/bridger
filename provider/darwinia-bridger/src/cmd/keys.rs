use crate::{error::Result, Settings};
use primitives::frame::{sudo::KeyStoreExt, technical_committee::MembersStoreExt};
use primitives::runtime::DarwiniaRuntime;
use substrate_subxt::ClientBuilder;

/// technical committee members
pub async fn exec() -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	let config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir

	let client = ClientBuilder::<DarwiniaRuntime>::new()
		.set_url(&config.darwinia.rpc)
		.build()
		.await?;
	let sudo = client.key(None).await?;
	// let sudo_ss58 = sudo.to_string();
	let technical_committee_members = client.members(None).await?;

	println!("sudo key: {:?}", sudo);
	println!(
		"technical committee members: {:?}",
		technical_committee_members
	);

	Ok(())
}
