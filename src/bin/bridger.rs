use darwinia_bridger::cmd;
use actix::System;

#[actix_rt::main]
async fn main() {
    log::info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    if let Err(err) = cmd::exec().await {
        log::error!("{}", err.to_string());
        System::current().stop();
    }
}
