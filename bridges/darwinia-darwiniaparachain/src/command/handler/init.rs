use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};

use bin_s2s::traits::{
    S2SParaBridgeRelayChainInfo, S2SParaBridgeSoloChainInfo, S2SSoloBridgeSoloChainInfo,
};
use support_common::config::{Config, Names};
use support_toolkit::convert::SmartCodecMapper;

use crate::types::{BridgeFlow, RawBridgeConfig};

pub async fn handle_init(bridge: BridgeFlow) -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-darwiniaparachain", "init bridge {:?}", bridge);
    let bridge_config: RawBridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
    init_bridge(bridge, bridge_config).await?;
    Ok(())
}

async fn init_bridge(bridge: BridgeFlow, bridge_config: RawBridgeConfig) -> color_eyre::Result<()> {
    let client_darwinia = bridge_config.darwinia.client().await?;
    let client_polkadot = bridge_config.polkadot.client().await?;
    let client_darwinia_parachain = bridge_config.darwinia_parachain.client().await?;
    let hash = match bridge {
        BridgeFlow::DarwiniaToDarwiniaParachain => {
            let initialization_data = client_darwinia.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_darwinia_parachain.initialize(expected_data).await?
        }
        BridgeFlow::PolkadotToDarwinia => {
            let initialization_data = client_polkadot.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_darwinia.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "darwinia-darwiniaparachain",
        "successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
