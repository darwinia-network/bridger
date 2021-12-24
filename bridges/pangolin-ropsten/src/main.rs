#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    Ok(())
}
