use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};

use bin_s2s::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
};
use support_common::config::{Config, Names};
use support_toolkit::convert::SmartCodecMapper;

use crate::types::{BridgeFlow, RawBridgeConfig};

pub async fn handle_init(bridge: BridgeFlow) -> color_eyre::Result<()> {
    tracing::info!(target: "crab-crabparachain", "init bridge {:?}", bridge);
    let bridge_config: RawBridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    init_bridge(bridge, bridge_config).await?;
    Ok(())
}

async fn init_bridge(bridge: BridgeFlow, bridge_config: RawBridgeConfig) -> color_eyre::Result<()> {
    let client_crab = bridge_config.crab.client().await?;
    let client_kusama = bridge_config.kusama.client().await?;
    let client_crab_parachain = bridge_config.crab_parachain.client().await?;
    let hash = match bridge {
        BridgeFlow::CrabToCrabparachain => {
            let initialization_data = client_crab.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_crab_parachain.initialize(expected_data).await?
        }
        BridgeFlow::KusamaToCrab => {
            let initialization_data = client_kusama.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_crab.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "crab-crabparachain",
        "successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
