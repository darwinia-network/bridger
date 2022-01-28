use colored::Colorize;

use client_pangolin::component::PangolinClientComponent;
use client_pangolin::types::runtime_types::darwinia_bridge_ethereum::EthereumRelayHeaderParcel;
use component_ethereum::errors::BizError;
use component_shadow::component::ShadowComponent;
use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_terminal::output;

use crate::bridge::PangolinRopstenConfig;
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
    _raw_json: Option<String>,
) -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

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
            shadow.parcel(block as usize + 1).await?.try_into()?
        }
        AffirmMode::Raw => {
            // let json = raw_json.ok_or_else(|| {
            //     BridgerError::Custom("You are missing `--raw` parameter".to_string())
            // })?;
            // serde_json::from_str(&json)
            //     .map_err(|e| BridgerError::Custom(format!("Failed to deserde json: {:?}", e)))?
            return Err(BridgerError::Custom("Not support this feature now".to_string()).into());
        }
    };

    let config_darwinia = bridge_config.darwinia;

    // Darwinia client
    let client = PangolinClientComponent::component(config_darwinia.clone()).await?;

    match mode {
        AffirmMode::Block => {
            let block_number = parcel.header.number;
            if parcel.parent_mmr_root.to_fixed_bytes() == [0u8; 32] {
                return Err(BizError::ParcelFromShadowIsEmpty(block.unwrap()).into());
            }
            let ex_hash = client.ethereum().affirm(parcel).await?;
            output::output_text(format!(
                "Affirmed ethereum block {} in extrinsic {:?}",
                block_number, ex_hash
            ));
        }
        AffirmMode::Raw => {
            return Err(BridgerError::Custom("Not support this feature now".to_string()).into());
        }
    }
    Ok(())
}

async fn handle_state() -> color_eyre::Result<()> {
    let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;

    // Darwinia client
    let client = PangolinClientComponent::component(bridge_config.darwinia).await?;

    let mut output = vec![];
    for (game_id, game) in client.ethereum().affirmations().await?.iter() {
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
