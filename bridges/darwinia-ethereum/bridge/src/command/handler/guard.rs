use client_darwinia::account::DarwiniaAccount;
use client_darwinia::component::DarwiniaSubxtComponent;
use client_darwinia::from_ethereum::Ethereum2Darwinia;
use component_shadow::ShadowComponent;
use std::sync::Arc;
use support_common::config::{Config, Names};
use support_terminal::output;

use crate::bridge::{DarwiniaEthereumConfig, Extrinsic};
use crate::service::guard::GuardService;

pub async fn handle_guard() -> color_eyre::Result<()> {
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

    let extrinsics = GuardService::extrinsics(
        ethereum_to_darwinia.clone(),
        from_ethereum_account.clone(),
        Arc::new(shadow),
    )
    .await?;
    for extrinsic in extrinsics {
        if let Extrinsic::GuardVote(pending_block_number, aye) = extrinsic {
            let ex_hash = ethereum_to_darwinia
                .vote_pending_relay_header_parcel(&from_ethereum_account, pending_block_number, aye)
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
