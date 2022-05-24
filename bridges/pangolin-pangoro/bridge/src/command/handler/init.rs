use lifeline::{Bus, Sender};

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::{BridgeName, InitBridge};

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangoro: ChainInfoConfig = bridge_config.pangoro;

    let (source_chain, target_chain) = match bridge {
        BridgeName::PangolinToPangoro => (config_pangolin, config_pangoro),
        BridgeName::PangoroToPangolin => (config_pangoro, config_pangolin),
    };

    init_bridge(InitBridge {
        bridge,
        source: source_chain,
        target: target_chain,
    })
    .await
}

async fn init_bridge(init_bridge: InitBridge) -> color_eyre::Result<()> {
    Ok(())
}
