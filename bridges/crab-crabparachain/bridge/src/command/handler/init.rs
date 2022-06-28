use abstract_bridge_s2s::client::S2SClientGeneric;
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "crab-crabparachain", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let config_crab: ChainInfoConfig = bridge_config.crab;
    let config_kusama: ChainInfoConfig = bridge_config.kusama;
    let config_crab_parachain: ChainInfoConfig = bridge_config.crab_parachain;

    init_bridge(bridge, config_crab, config_kusama, config_crab_parachain).await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_crab: ChainInfoConfig,
    config_kusama: ChainInfoConfig,
    config_crab_parachain: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_crab = config_crab.to_crab_client().await?;
    let client_kusama = config_kusama.to_kusama_client().await?;
    let client_crab_parachain = config_crab_parachain.to_crab_parachain_client().await?;
    let hash = match bridge {
        BridgeName::CrabToCrabParachain => {
            let initialization_data = client_crab.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_crab_parachain.initialize(expected_data).await?
        }
        BridgeName::KusamaToCrab => {
            let initialization_data = client_kusama.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_crab.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "crab-crabparachain",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
