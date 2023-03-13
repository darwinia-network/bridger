use bin_e2e::config::BridgeConfig;
use bin_e2e::task::BridgeTask;
use subquery::types::BridgeName;
use support_common::config::{Config, Names};
use thegraph::types::LikethChain;

use crate::bridge::BridgeConfig as RawBridgeConfig;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge pangolin-goerli");
    let raw_config: RawBridgeConfig = Config::restore(Names::BridgePangolinGoerli)?;
    let bridge_config = BridgeConfig {
        name: BridgeName::PangolinGoerli.name().into(),
        general: raw_config.general,
        darwinia_evm: raw_config.pangolin_evm,
        substrate_client: raw_config.pangolin_substrate.to_substrate_client().await?,
        ethereum: raw_config.goerli,
        beacon: raw_config.beacon,
        substrate_index: raw_config
            .index
            .to_substrate_subquery(BridgeName::PangolinGoerli),
        evm_index: raw_config.index.to_evm_thegraph(LikethChain::Pangolin)?,
    };
    let _manager = BridgeTask::new(bridge_config)?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
