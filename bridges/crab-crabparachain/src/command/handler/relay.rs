use bin_s2s::bridge::config::solo_with_para::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::solo_with_para::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;
use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "crab-crabparachain", "start bridge crab-crabparachain");
    let raw_bridge_config: RawBridgeConfig = Config::restore(Names::BridgeCrabCrabParachain)?;
    let raw_config_index = raw_bridge_config.index;
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            solo: raw_bridge_config.crab,
            para: raw_bridge_config.crab_parachain,
            relay: raw_bridge_config.kusama,
        },
        relay: raw_bridge_config.relay,
        index: IndexConfig {
            solo: BasicSubqueryInfo::new(BridgeName::CrabCrabParachain, raw_config_index.crab),
            para: BasicSubqueryInfo::new(
                BridgeName::CrabCrabParachain,
                raw_config_index.crab_parachain,
            ),
            relay: BasicSubqueryInfo::new(BridgeName::CrabCrabParachain, raw_config_index.kusama),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
