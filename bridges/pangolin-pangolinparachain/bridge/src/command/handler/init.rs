use abstract_bridge_s2s::client::S2SClientGeneric;
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangolinparachain", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachain)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_rococo: ChainInfoConfig = bridge_config.rococo;
    let config_pangolin_parachain: ChainInfoConfig = bridge_config.pangolin_parachain;

    init_bridge(
        bridge,
        config_pangolin,
        config_rococo,
        config_pangolin_parachain,
    )
    .await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_pangolin: ChainInfoConfig,
    config_rococo: ChainInfoConfig,
    config_pangolin_parachain: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_pangolin = config_pangolin.to_pangolin_client().await?;
    let client_rococo = config_rococo.to_rococo_client().await?;
    let client_pangolin_parachain = config_pangolin_parachain
        .to_pangolin_parachain_client()
        .await?;
    let hash = match bridge {
        BridgeName::PangolinToPangolinParachain => {
            let initialization_data = client_pangolin.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin_parachain.initialize(expected_data).await?
        }
        BridgeName::RococoToPangolin => {
            let initialization_data = client_rococo.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "pangolin-pangolinparachain",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
