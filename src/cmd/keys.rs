use crate::{
    error::Result,
    Config,
};
use substrate_subxt::ClientBuilder;
use primitives::runtime::DarwiniaRuntime;
use primitives::frame::{
    technical_committee::MembersStoreExt,
    sudo::KeyStoreExt,
};

/// technical committee members
pub async fn exec() -> Result<()> {
    std::env::set_var("RUST_LOG", "info,darwinia_bridger");
    env_logger::init();

    let config = Config::new(&Config::default_data_dir()?)?;  // TODO: add --data-dir

    let client = ClientBuilder::<DarwiniaRuntime>::new()
        .set_url(&config.node)
        .build()
        .await?;
    let sudo = client.key(None).await?;
    // let sudo_ss58 = sudo.to_string();
    let technical_committee_members = client.members(None).await?;

    println!("sudo key: {:?}", sudo);
    println!("technical committee members: {:?}", technical_committee_members);

    Ok(())
}
