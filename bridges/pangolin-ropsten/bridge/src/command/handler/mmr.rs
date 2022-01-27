use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::EthereumAccount;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;
use crate::command::types::MmrOpts;

pub async fn handle_mmr(opts: MmrOpts) -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let network = opts.network;
    let mmrblock = opts.mmrblock;
    let config_darwinia = bridge_config.darwinia;

    // Web3
    let config_web3 = bridge_config.web3;

    // Darwinia client
    let client = PangolinClientComponent::component(config_darwinia.clone()).await?;

    let ethereum_account = EthereumAccount::new(
        config_web3.endpoint,
        config_darwinia.ecdsa_authority_private_key,
    );

    let tx = client
        .ethereum()
        .ecdsa_sign_and_submit_signed_mmr_root(ethereum_account, mmrblock as u32)
        .await?;

    output::output_text(format!("{}", tx));
    Ok(())
}
