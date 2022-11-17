use bin_s2s::bridge::config::solo_with_para::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::solo_with_para::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;
use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-darwiniaparachain", "start bridge darwinia-darwiniaparachain");
    let raw_bridge_config: RawBridgeConfig =
        Config::restore(Names::BridgeDarwiniaDarwiniaParachain)?;
    let raw_config_index = raw_bridge_config.index;
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            solo: raw_bridge_config.darwinia,
            para: raw_bridge_config.darwinia_parachain,
            relay: raw_bridge_config.polkadot,
        },
        relay: raw_bridge_config.relay,
        index: IndexConfig {
            solo: BasicSubqueryInfo::new(
                BridgeName::DarwiniaDarwiniaParachain,
                raw_config_index.darwinia,
            ),
            para: BasicSubqueryInfo::new(
                BridgeName::DarwiniaDarwiniaParachain,
                raw_config_index.darwinia_parachain,
            ),
            relay: BasicSubqueryInfo::new(
                BridgeName::DarwiniaDarwiniaParachain,
                raw_config_index.polkadot,
            ),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
