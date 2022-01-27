use client_pangolin::component::PangolinClientComponent;
use substrate_subxt::ClientBuilder;

use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;

pub async fn handle_keys() -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let config_darwinia = bridge_config.darwinia;

    let client = PangolinClientComponent::component(config_darwinia).await?;
    let sudo = client.runtime().storage().sudo().key(None).await?;
    let technical_committee_members = client
        .runtime()
        .storage()
        .technical_committee()
        .members(None)
        .await?;

    let msgs = vec![
        format!("sudo key: {:?}", sudo),
        format!(
            "technical committee members: {:?}",
            technical_committee_members
        ),
    ];

    output::output_text(msgs.join("\n"));
    Ok(())
}
