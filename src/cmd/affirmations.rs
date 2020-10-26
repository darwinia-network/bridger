use crate::{
    result::Result,
    Config,
};
use std::sync::Arc;
use crate::api::Darwinia;

/// get all affirmations
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let config = Config::new(None)?;
    let darwinia = Arc::new(Darwinia::new(&config).await?);
    info!("Init API succeed!");

    for (game_id, game) in darwinia.affirmations().await?.iter() {
        println!("--- GAME {} ---", game_id);
        for (round_id, affirmations) in game.iter() {
            println!("ROUND {}", round_id);
            for affirmation in affirmations {
                println!("affirmation: {:?}", affirmation);
            }
        }
    }

    Ok(())
}
