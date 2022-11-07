use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};
use support_toolkit::convert::SmartCodecMapper;

use support_common::config::{Config, Names};

use crate::bridge::{BridgeConfig, ChainInfoConfig};
use crate::types::BridgeName;

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-darwiniaparachain", "Init bridge {:?}", bridge);
    let bridge_config: BridgeConfig = Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
    let config_darwinia: ChainInfoConfig = bridge_config.darwinia;
    let config_polkadot: ChainInfoConfig = bridge_config.polkadot;
    let config_darwinia_parachain: ChainInfoConfig = bridge_config.darwinia_parachain;

    init_bridge(
        bridge,
        config_darwinia,
        config_polkadot,
        config_darwinia_parachain,
    )
    .await?;
    Ok(())
}

async fn init_bridge(
    bridge: BridgeName,
    config_darwinia: ChainInfoConfig,
    config_polkadot: ChainInfoConfig,
    config_darwinia_parachain: ChainInfoConfig,
) -> color_eyre::Result<()> {
    let client_darwinia = config_darwinia.to_darwinia_client().await?;
    let client_polkadot = config_polkadot.to_polkadot_client().await?;
    let client_darwinia_parachain = config_darwinia_parachain
        .to_darwinia_parachain_client()
        .await?;
    let hash = match bridge {
        BridgeName::DarwiniaToDarwiniaParachain => {
            let initialization_data = client_darwinia.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_darwinia_parachain.initialize(expected_data).await?
        }
        BridgeName::PolkadotToDarwinia => {
            let initialization_data = client_polkadot.prepare_initialization_data().await?;
            let expected_data = SmartCodecMapper::map_to(&initialization_data)?;
            client_darwinia.initialize(expected_data).await?
        }
    };
    tracing::info!(
        target: "darwinia-darwiniaparachain",
        "Successes to sent init transaction: {:?}",
        hash,
    );
    Ok(())
}
