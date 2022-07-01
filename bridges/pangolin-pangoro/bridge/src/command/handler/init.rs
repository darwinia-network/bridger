use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_pangoro: ChainInfoConfig = bridge_config.pangoro;

    init_bridge(bridge, config_pangolin, config_pangoro).await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_pangolin: ChainInfoConfig,
    config_pangoro: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_pangolin = config_pangolin.to_pangolin_client().await?;
    let client_pangoro = config_pangoro.to_pangoro_client().await?;
    let hash = match bridge {
        BridgeName::PangolinToPangoro => {
            let initialization_data = client_pangolin.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangoro.initialize(expected_data).await?
        }
        BridgeName::PangoroToPangolin => {
            let initialization_data = client_pangoro.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "pangolin-pangoro",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
