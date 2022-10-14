use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangolinparachainalpha", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgePangolinPangolinParachainAlpha)?;
    let config_pangolin: ChainInfoConfig = bridge_config.pangolin;
    let config_moonbase: ChainInfoConfig = bridge_config.moonbase;
    let config_pangolin_parachain: ChainInfoConfig = bridge_config.pangolin_parachain_alpha;

    init_bridge(
        bridge,
        config_pangolin,
        config_moonbase,
        config_pangolin_parachain,
    )
    .await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_pangolin: ChainInfoConfig,
    config_moonbase: ChainInfoConfig,
    config_pangolin_parachain: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_pangolin = config_pangolin.to_pangolin_client().await?;
    let client_moonbase = config_moonbase.to_moonbase_client().await?;
    let client_pangolin_parachain = config_pangolin_parachain
        .to_pangolin_parachain_client()
        .await?;
    let hash = match bridge {
        BridgeName::PangolinToPangolinParachainAlpha => {
            let initialization_data = client_pangolin.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin_parachain.initialize(expected_data).await?
        }
        BridgeName::MoonbaseToPangolin => {
            let initialization_data = client_moonbase.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "pangolin-pangolinparachainalpha",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
