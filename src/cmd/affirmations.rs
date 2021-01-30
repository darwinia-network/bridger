use crate::{
	error::{
        Result,
    },
	Config,
};

use darwinia::{
    Darwinia,
    Ethereum2Darwinia,
};

/// get all affirmations
pub async fn exec() -> Result<()> {
	std::env::set_var("RUST_LOG", "info,darwinia_bridger");
	env_logger::init();

	// apis
	let config = Config::new(&Config::default_data_dir()?)?;
	let darwinia = Darwinia::new(&config.node).await?;
 	let ethereum2darwinia = Ethereum2Darwinia::new(darwinia.clone());
 
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
