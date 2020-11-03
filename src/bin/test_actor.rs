#![feature(async_closure)]
use darwinia_bridger::{Config};
use std::sync::Arc;
use darwinia_bridger::api::{Shadow, Darwinia};
use actix::{Actor, System};

// use darwinia_bridger::service::{RelayService, relay::{MsgExecute, MsgBlockNumber}};

use darwinia_bridger::service::{RedeemService, EthereumTransactionHash, redeem::MsgEthereumTransaction};
use darwinia_bridger::memcache::EthereumTransaction;
use web3::types::H256;

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info, debug, darwinia_bridger");
    env_logger::init();

    let config = Config::new(None).unwrap();
    let shadow = Arc::new(Shadow::new(&config));
    let darwinia = Arc::new(Darwinia::new(&config).await.unwrap());

    // let last_confirmed = darwinia.last_confirmed().await.unwrap();
    // println!("last confirmed: #{}", last_confirmed);
    // let my_relay_service = RelayService::new(shadow, darwinia, 100).start();
    // let _ = my_relay_service.send(MsgBlockNumber(101u64)).await;
    // let res = my_relay_service.send(MsgExecute{}).await; // <- send message and get future for result
    // println!("{:?}", res);

    let my_redeem_service = RedeemService::new(shadow, darwinia, 5).start();
    let tx = EthereumTransaction {
        tx_hash: EthereumTransactionHash::Token(H256::zero()),
        block_hash: H256::zero(),
        block: 0,
        index: 0
    };
    let msg = MsgEthereumTransaction { tx };
    let _ = my_redeem_service.send(msg).await;

    //
    tokio::signal::ctrl_c().await.unwrap();
    println!("Ctrl-C received, shutting down");
    System::current().stop();
}
