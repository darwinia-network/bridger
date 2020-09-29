use darwinia_bridger::cmd;

#[tokio::main]
async fn main() {
    cmd::exec().await.unwrap();
}
