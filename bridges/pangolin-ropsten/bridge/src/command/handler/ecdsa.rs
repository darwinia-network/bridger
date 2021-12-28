use client_pangolin::account::DarwiniaAccount;
use client_pangolin::component::DarwiniaSubxtComponent;
use client_pangolin::to_ethereum::Darwinia2Ethereum;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;
use crate::command::types::EcdsaOpts;

pub async fn handle_ecdsa(opts: EcdsaOpts) -> color_eyre::Result<()> {
    let message = opts.message;

    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    let config_darwinia = bridge_config.darwinia;
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

    let message = array_bytes::hex2bytes(&message[2..])
        .map_err(|_| BridgerError::Custom("message[2..]".into()))?;
    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&message);
    darwinia_to_ethereum
        .ecdsa_sign_and_submit_signed_authorities(&to_ethereum_account, buffer)
        .await?;

    output::output_text("Success");
    Ok(())
}
