mod bus;
mod message;
mod service;
mod task;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_initialize::init()?;
    Ok(())
}
