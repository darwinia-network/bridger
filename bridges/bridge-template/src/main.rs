#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_initialize::init()?;

    let mut times = 0;
    loop {
        times += 1;
        if times > 5 {
            return Err(color_eyre::Report::msg("Test error"));
        }
        tracing::debug!("bridge-template Count: {}", times);
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
