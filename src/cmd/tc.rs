use crate::{
    result::Result,
    Config,
};
use substrate_subxt::ClientBuilder;
use primitives::runtime::DarwiniaRuntime;
use primitives::frame::collective::MembersStoreExt;

/// technical committee members
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let config = Config::new(None)?;

    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(&config.node)
        .build()
        .await?;
    let technical_committee_members = client.members(None).await?;

    println!("{:?}", technical_committee_members);

    Ok(())
}
