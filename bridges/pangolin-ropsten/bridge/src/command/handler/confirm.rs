use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::{darwinia_bridge_ethereum, pangolin_runtime};
use component_shadow::component::ShadowComponent;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;
use crate::command::types::ConfirmOpts;

pub async fn handle_confirm(opts: ConfirmOpts) -> color_eyre::Result<()> {
    let block = opts.block;

    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let config_darwinia = bridge_config.darwinia;

    // Shadow
    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
    )?;

    // Darwinia client
    let client = PangolinClientComponent::component(config_darwinia).await?;

    let parcel = shadow.parcel(block as usize).await?;

    let tx = client
        .runtime()
        .tx()
        .sudo()
        .sudo(pangolin_runtime::Call::EthereumRelay(
            darwinia_bridge_ethereum::Call::set_confirmed_parcel {
                ethereum_relay_header_parcel: parcel.try_into()?,
            },
        ))
        .sign_and_submit(client.account().signer())
        .await?;

    let msg = format!("Set confirmed block {} succeed! {:?}", block, tx);
    output::output_text(msg);
    Ok(())
}
