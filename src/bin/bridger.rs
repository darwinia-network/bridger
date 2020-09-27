use darwinia_bridger::cmd;

#[async_std::main]
async fn main() {
    cmd::exec().await.unwrap();
}
