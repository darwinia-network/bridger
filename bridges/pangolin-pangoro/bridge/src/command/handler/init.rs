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
            let initialization_data = client_pangoro.prepare_initialization_data().await?;
            println!("{:?}", initialization_data);
            let initialization_data = client_pangolin.prepare_initialization_data().await?;
            println!("{:?}", initialization_data);
        }
        BridgeName::PangolinToPangoro => {}
    }
    Ok(())
}
