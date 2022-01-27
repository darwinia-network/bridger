use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::EthereumAccount;
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
    let client = PangolinClientComponent::component(config_darwinia.clone()).await?;

    let ethereum_account = EthereumAccount::new(
        config_web3.endpoint,
        config_darwinia.ecdsa_authority_private_key,
    );

    let message = array_bytes::hex2bytes(&message[2..])
        .map_err(|_| BridgerError::Custom("message[2..]".into()))?;
    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&message);

    let tx = client
        .ethereum()
        .ecdsa_sign_and_submit_signed_authorities(ethereum_account, buffer)
        .await?;

    output::output_text(format!("{:?}", tx));
    output::output_text("Success");
    Ok(())
}
