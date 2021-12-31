use client_darwinia::account::DarwiniaAccount;
use client_darwinia::component::DarwiniaSubxtComponent;
use client_darwinia::from_ethereum::Ethereum2Darwinia;
use component_shadow::ShadowComponent;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::DarwiniaEthereumConfig;
use crate::command::types::ConfirmOpts;

pub async fn handle_confirm(opts: ConfirmOpts) -> color_eyre::Result<()> {
    let block = opts.block;

    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let config_darwinia = bridge_config.darwinia;

    // Shadow
    let shadow = ShadowComponent::component(
        bridge_config.shadow,
        bridge_config.ethereum,
        bridge_config.web3,
    )?;

    // Darwinia client
    let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
    let ethereum_to_darwinia = Ethereum2Darwinia::new(darwinia);

    // Account
    let darwinia_account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );
    let from_ethereum_account = client_darwinia::from_ethereum::Account::new(darwinia_account);

    let parcel = shadow.parcel(block as usize).await?;

    ethereum_to_darwinia
        .set_confirmed_parcel(&from_ethereum_account, parcel)
        .await?;

    let msg = format!("Set confirmed block {} succeed!", block);
    output::output_text(msg);
    Ok(())
}
