use crate::{
	api::{darwinia_api, Shadow},
	error::Result,
	Settings,
};
use rpassword::prompt_password_stdout;

/// Run the bridger
pub async fn exec(block: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	let mut config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	if config.encrypted {
		let passwd = prompt_password_stdout("Please enter password:")?;
		config.decrypt(&passwd)?;
	}
	let shadow = Shadow::new(&config);
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia);
	let darwinia_account = darwinia_api::get_darwinia_account(&config);
	let e2d_account = darwinia_api::get_e2d_account(darwinia_account);
	info!("Init darwinia API succeed!");
	let parcel = shadow.parcel(block as usize).await?;
	info!("Init shadow API succeed!");
	ethereum2darwinia
		.set_confirmed_parcel(&e2d_account, parcel)
		.await?;
	info!("Set confirmed block {} succeed!", block);
	Ok(())
}
