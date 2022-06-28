use abstract_bridge_s2s::client::{S2SClientGeneric, S2SClientRelay};
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-crab", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let config_crab: ChainInfoConfig = bridge_config.crab;
    let config_darwinia: ChainInfoConfig = bridge_config.darwinia;

    init_bridge(bridge, config_crab, config_darwinia).await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_crab: ChainInfoConfig,
    config_darwinia: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_crab = config_crab.to_crab_client().await?;
    let client_darwinia = config_darwinia.to_darwinia_client().await?;
    let hash = match bridge {
        BridgeName::CrabToDarwinia => {
            let initialization_data = client_crab.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_darwinia.initialize(expected_data).await?
        }
        BridgeName::DarwiniaToCrab => {
            let initialization_data = client_darwinia.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_crab.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "darwinia-crab",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
