use darwinia_bridger::cmd;
use actix::System;

#[actix_rt::main]
async fn main() {
    if let Err(err) = cmd::exec().await {
        log::error!("{:?}", err);
    }

    tokio::signal::ctrl_c().await.unwrap();
    log::info!("Ctrl-C received, shutting down");
    System::current().stop();
}
