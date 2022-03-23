use client_darwinia::component::DarwiniaClientComponent;

use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::DarwiniaEthereumConfig;

pub async fn handle_keys() -> color_eyre::Result<()> {
    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let config_darwinia = bridge_config.darwinia;

    let client = DarwiniaClientComponent::component(config_darwinia).await?;
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
