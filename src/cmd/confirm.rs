use crate::{
	api::{Darwinia, Shadow},
	error::Result,
	Settings,
};

/// Run the bridger
pub async fn exec(block: u64) -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	let config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	let shadow = Shadow::new(&config);
	let darwinia = Darwinia::new(&config).await?;
	info!("Init darwinia API succeed!");
	let parcel = shadow.parcel(block as usize).await?;
	info!("Init shadow API succeed!");
	darwinia.set_confirmed_parcel(parcel).await?;
	info!("Set confirmed block {} succeed!", block);
	Ok(())
}
