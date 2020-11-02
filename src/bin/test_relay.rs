#![feature(async_closure)]
use darwinia_bridger::{Config};
use std::sync::Arc;
use darwinia_bridger::api::{Shadow, Darwinia};
use darwinia_bridger::service::RelayService;
use actix::Actor;
use darwinia_bridger::service::relay::MsgExecute;
use tokio::time::Duration;

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info, debug, darwinia_bridger");
    env_logger::init();

    let config = Config::new(None).unwrap();
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia = Arc::new(Darwinia::new(&config).await.unwrap());

    let last_confirmed = darwinia.last_confirmed().await.unwrap();
    println!("last confirmed: #{}", last_confirmed);
    let my_relay_service = RelayService::new(shadow, darwinia, 100).start();
    let res = my_relay_service.send(MsgExecute{}).await; // <- send message and get future for result

}
