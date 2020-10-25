use crate::{
    result::Result,
    Config,
};
use substrate_subxt::ClientBuilder;
use primitives::runtime::DarwiniaRuntime;
use primitives::frame::ethereum::{
    game::AffirmationsStoreExt
};

/// technical committee members
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let config = Config::new(None)?;

    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(&config.node)
        .build()
        .await?;
    let mut iter = client.affirmations_iter(None).await?;
    loop {
        if let Some((mut storage_key, affirmations)) = iter.next().await? {
            let game_id: &mut [u8] = &mut storage_key.0[32..40];
            game_id.reverse();
            let game_id_result = u64::from_str_radix(hex::encode(game_id).as_str(), 16).unwrap();

            let round_id: &mut [u8] = &mut storage_key.0[40..44];
            round_id.reverse();
            let round_id_result = u64::from_str_radix(hex::encode(round_id).as_str(), 16).unwrap();

            println!("-------------------");
            println!("game: {}, round: {}", game_id_result, round_id_result);
            println!("affirmations: {:?}", affirmations);
        } else {
            break;
        }
    }

    Ok(())
}
