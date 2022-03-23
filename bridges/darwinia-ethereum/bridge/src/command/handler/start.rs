use crate::bridge::DarwiniaEthereumTask;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge darwinia-ropsten");
    let _task = DarwiniaEthereumTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
