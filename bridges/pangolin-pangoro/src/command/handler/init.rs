use bin_s2s::traits::{S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo};
use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};

use support_common::config::{Config, Names};
use support_toolkit::convert::SmartCodecMapper;

use crate::types::{BridgeFlow, RawBridgeConfig};

pub async fn handle_init(bridge: BridgeFlow) -> color_eyre::Result<()> {
    tracing::info!(target: "pangolin-pangoro", "init bridge {:?}", bridge);
    let bridge_config: RawBridgeConfig = Config::restore(Names::BridgePangolinPangoro)?;
    init_bridge(bridge, bridge_config).await?;
    Ok(())
}

async fn init_bridge(bridge: BridgeFlow, bridge_config: RawBridgeConfig) -> color_eyre::Result<()> {
    let client_rococo = bridge_config.rococo.client().await?;
    let client_moonbase = bridge_config.moonbase.client().await?;
    let client_pangolin = bridge_config.pangolin.client().await?;
    let client_pangoro = bridge_config.pangoro.client().await?;
    let hash = match bridge {
        BridgeFlow::PangoroToPangolin => {
            let initialization_data = client_moonbase.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangolin.initialize(expected_data).await?
        }
        BridgeFlow::PangolinToPangoro => {
            let initialization_data = client_rococo.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_pangoro.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "pangolin-pangoro",
        "successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
