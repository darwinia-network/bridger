use crate::{api::darwinia_api, error::Result, Settings};

/// get all affirmations
pub async fn exec() -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Settings::new(&Settings::default_data_dir()?)?; // TODO: add --data-dir
	let darwinia = darwinia_api::get_darwinia_instance(&config).await?;
	let ethereum2darwinia = darwinia_api::get_e2d_instance(darwinia);

	info!("Init API succeed!");

	for (game_id, game) in ethereum2darwinia.affirmations().await?.iter() {
		println!("--- GAME {} ---", game_id);
		for (round_id, affirmations) in game.iter() {
			println!("ROUND {}", round_id);
			for affirmation in affirmations {
				println!("affirmation: {}", affirmation);
			}
		}
	}

	Ok(())
}
