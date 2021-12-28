use crate::bridge::PangolinRopstenTask;

pub async fn handle_start() -> color_eyre::Result<()> {
    tracing::info!("Start bridge pangolin-ropsten");
    let _task = PangolinRopstenTask::new().await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
