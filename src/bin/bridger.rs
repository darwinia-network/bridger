use darwinia_bridger::cmd;

#[tokio::main]
async fn main() {
    if let Ok(err) = cmd::exec().await {
        log::error!("{:?}", err);
    }
}
