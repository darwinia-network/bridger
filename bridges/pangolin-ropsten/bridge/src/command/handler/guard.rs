use client_pangolin::component::PangolinClientComponent;
use shadow_liketh::component::ShadowComponent;
use shadow_liketh::types::BridgeName;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::{Extrinsic, PangolinRopstenConfig};
use crate::service::guard::GuardService;

pub async fn handle_guard() -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let config_darwinia = bridge_config.darwinia;

    // Shadow
    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
        BridgeName::PangolinRopsten,
    )?;

    // Darwinia client
    let client = PangolinClientComponent::component(config_darwinia.clone()).await?;

    let extrinsics = GuardService::extrinsics(&client, &shadow).await?;
    for extrinsic in extrinsics {
        if let Extrinsic::GuardVote(pending_block_number, aye) = extrinsic {
            let ex_hash = client
                .runtime()
                .tx()
                .ethereum_relay()
                .vote_pending_relay_header_parcel(pending_block_number, aye)
                .sign_and_submit(client.account().signer())
                .await?;
            if aye {
                output::output_text(format!(
                    "Voted to approve: {}, ex hash: {:?}",
                    pending_block_number, ex_hash
                ));
            } else {
                output::output_text(format!(
                    "Voted to reject: {}, ex hash: {:?}",
                    pending_block_number, ex_hash
                ));
            }
        }
    }
    Ok(())
}
