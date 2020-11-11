use darwinia_bridger::cmd;
use actix::System;

#[actix_rt::main]
async fn main() {
    if let Err(err) = cmd::exec().await {
        log::error!("{}", err.to_string());
        System::current().stop();
    }
}
