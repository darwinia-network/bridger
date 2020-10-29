use darwinia_bridger::cmd;

#[tokio::main]
async fn main() {
    if let Err(err) = cmd::exec().await {
        log::error!("{:?}", err);
    }
}
