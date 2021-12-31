use client_pangolin::account::DarwiniaAccount;
use client_pangolin::component::DarwiniaSubxtComponent;
use client_pangolin::to_ethereum::Darwinia2Ethereum;
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
    let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
    let darwinia_to_ethereum = Darwinia2Ethereum::new(darwinia);

    // Account
    let darwinia_account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );

    let to_ethereum_account = client_pangolin::to_ethereum::Account::new(
        darwinia_account,
        config_darwinia.ecdsa_authority_private_key,
        config_web3.endpoint,
    );

    let tx = darwinia_to_ethereum
        .ecdsa_sign_and_submit_signed_mmr_root(
            &to_ethereum_account,
            network.to_string(),
            mmrblock as u32,
        )
        .await?;

    output::output_text(format!("{}", tx));
    Ok(())
}
