use client_pangolin::component::PangolinClientComponent;
use client_pangoro::component::PangoroClientComponent;
use lifeline::{Bus, Sender};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangoro: ChainInfoConfig = bridge_config.pangoro;

    // let (source_chain, target_chain) = match bridge {
    //     BridgeName::PangolinToPangoro => (config_pangolin, config_pangoro),
    //     BridgeName::PangoroToPangolin => (config_pangoro, config_pangolin),
    // };

    init_bridge(bridge, config_pangolin, config_pangoro).await?;
    loop {}
}

async fn init_bridge(
    bridge: BridgeName,
    config_pangolin: ChainInfoConfig,
    config_pangoro: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_pangolin = PangolinClientComponent::component(config_pangolin.into()).await?;
    let client_pangoro = PangoroClientComponent::component(config_pangoro.into()).await?;
    match bridge {
        BridgeName::PangoroToPangolin => {
            let mut subscription = client_pangoro.subscribe_justification().await?;
            let justification = subscription
                .next()
                .await
                .ok_or_else(|| BridgerError::Custom("The subscribe is closed".to_string()))??;
            let justification: bp_header_chain::justification::GrandpaJustification<
                <bp_pangolin::Pangolin as bp_runtime::Chain>::Header,
            > = codec::Decode::decode(&mut &justification.0[..])
                .map_err(|err| BridgerError::Custom(format!("Wrong justification: {:?}", err)))?;

            let (initial_header_hash, initial_header_number) = (
                justification.commit.target_hash,
                justification.commit.target_number,
            );
            println!("{:?}", justification);
        }
        BridgeName::PangolinToPangoro => {}
    }
    Ok(())
}
