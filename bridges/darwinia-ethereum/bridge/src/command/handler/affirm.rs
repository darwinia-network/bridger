use colored::Colorize;

use client_darwinia::account::DarwiniaAccount;
use client_darwinia::component::DarwiniaSubxtComponent;
use client_darwinia::from_ethereum::Ethereum2Darwinia;
use component_ethereum::errors::BizError;
use component_shadow::ShadowComponent;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_ethereum::block::EthereumHeader;
use support_ethereum::parcel::EthereumRelayHeaderParcel;
use support_terminal::output;

use crate::bridge::DarwiniaEthereumConfig;
use crate::command::types::{AffirmMode, AffirmOpts};

pub async fn handle_affirm(opts: AffirmOpts) -> color_eyre::Result<()> {
    match opts {
        AffirmOpts::Do {
            mode,
            block,
            raw_json,
        } => handle_do(mode, block, raw_json).await,
        AffirmOpts::State => handle_state().await,
    }
}

async fn handle_do(
    mode: AffirmMode,
    block: Option<u64>,
    raw_json: Option<String>,
) -> color_eyre::Result<()> {
    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

    let parcel: EthereumRelayHeaderParcel = match mode {
        AffirmMode::Block => {
            let block = block.ok_or_else(|| {
                BridgerError::Custom("You are missing `--block` parameter".to_string())
            })?;
            // Shadow
            let shadow = ShadowComponent::component(
                bridge_config.shadow,
                bridge_config.ethereum,
                bridge_config.web3,
            )?;
            shadow.parcel(block as usize + 1).await?
        }
        AffirmMode::Raw => {
            let json = raw_json.ok_or_else(|| {
                BridgerError::Custom("You are missing `--raw` parameter".to_string())
            })?;
            serde_json::from_str(&json)
                .map_err(|e| BridgerError::Custom(format!("Failed to deserde json: {:?}", e)))?
        }
    };

    let config_darwinia = bridge_config.darwinia;

    // Darwinia client
    let darwinia = DarwiniaSubxtComponent::component(config_darwinia.clone()).await?;
    let ethereum_to_darwinia = Ethereum2Darwinia::new(darwinia);

    // Account
    let darwinia_account = DarwiniaAccount::new(
        config_darwinia.relayer_private_key,
        config_darwinia.relayer_real_account,
    );
    let from_ethereum_account = client_darwinia::from_ethereum::Account::new(darwinia_account);

    match mode {
        AffirmMode::Block => {
            let block_number = parcel.header.number;
            if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
                return Err(BizError::ParcelFromShadowIsEmpty(block.unwrap()).into());
            }
            let ex_hash = ethereum_to_darwinia
                .affirm(&from_ethereum_account, parcel)
                .await?;
            output::output_text(format!(
                "Affirmed ethereum block {} in extrinsic {:?}",
                block_number, ex_hash
            ));
        }
        AffirmMode::Raw => {
            // affirm
            let hash = ethereum_to_darwinia
                .affirm(&from_ethereum_account, parcel)
                .await?;
            output::output_text(format!("Extrinsic hash: {:?}", hash));
        }
    }
    Ok(())
}

async fn handle_state() -> color_eyre::Result<()> {
    let bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;

    // Darwinia client
    let darwinia = DarwiniaSubxtComponent::component(bridge_config.darwinia).await?;
    let ethereum_to_darwinia = Ethereum2Darwinia::new(darwinia);

    let mut output = vec![];
    for (game_id, game) in ethereum_to_darwinia.affirmations().await?.iter() {
        output.push(format!("{}", &format!("--- GAME {} ---", game_id).bold()));
        for (round_id, affirmations) in game.iter() {
            output.push(format!("ROUND {}", round_id));
            for affirmation in affirmations {
                output.push(format!("{} {:?}\n", "affirmation:".blue(), affirmation));
            }
        }
    }
    if output.is_empty() {
        output::output_text("Not have affirm data");
    } else {
        output::output_text(output.join("\n"));
    }
    Ok(())
}
