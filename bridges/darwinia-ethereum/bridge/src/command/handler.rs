use crate::bridge::BridgeTask;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge darwinia-eth");
    let _manager = BridgeTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
