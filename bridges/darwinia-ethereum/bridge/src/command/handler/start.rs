use support_common::config::{Config, Names};

use crate::bridge::{DarwiniaEthereumConfig, DarwiniaEthereumTask};

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge darwinia-ethereum");
    // check config
    let _bridge_config: DarwiniaEthereumConfig = Config::restore(Names::BridgeDarwiniaEthereum)?;
    let _task = DarwiniaEthereumTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
