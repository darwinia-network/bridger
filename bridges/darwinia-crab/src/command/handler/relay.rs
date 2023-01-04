use bin_s2s::bridge::config::para_with_para::{BridgeConfig, ChainConfig, IndexConfig};
use bin_s2s::bridge::config::ParaWithParaConfig;
use bin_s2s::bridge::para_with_para::BridgeTask;
use bin_s2s::types::BasicSubqueryInfo;

use support_common::config::{Config, Names};
use support_types::mark::BridgeName;

use crate::types::RawBridgeConfig;

pub async fn handle_relay() -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-crab", "start bridge darwinia-crab");
    let raw_bridge_config: RawBridgeConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let raw_config_index = raw_bridge_config.index;
    let (source_para_id, target_para_id) = (
        raw_bridge_config.darwinia.para_id,
        raw_bridge_config.crab.para_id,
    );
    let bridge_config = BridgeConfig {
        chain: ChainConfig {
            source_para: raw_bridge_config.darwinia,
            source_relay: raw_bridge_config.polkadot,
            target_para: raw_bridge_config.crab,
            target_relay: raw_bridge_config.kusama,
        },
        relay: raw_bridge_config.relay,
        para_config: ParaWithParaConfig {
            source_para_id,
            target_para_id,
        },
        index: IndexConfig {
            source_para: BasicSubqueryInfo::new(
                BridgeName::DarwiniaCrab,
                raw_config_index.darwinia,
            ),
            source_relay: BasicSubqueryInfo::new(
                BridgeName::DarwiniaCrab,
                raw_config_index.polkadot,
            ),
            target_para: BasicSubqueryInfo::new(BridgeName::DarwiniaCrab, raw_config_index.crab),
            target_relay: BasicSubqueryInfo::new(BridgeName::DarwiniaCrab, raw_config_index.kusama),
        },
    };
    let _task = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
