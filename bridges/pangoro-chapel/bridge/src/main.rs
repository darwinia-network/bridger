use structopt::StructOpt;

use crate::command::types::Opts;
use web3::types::{BlockId, BlockNumber, U64};

mod bridge;
mod cli;
mod command;
mod service;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;

    let transport = web3::transports::Http::new("https://data-seed-prebsc-1-s1.binance.org:8545/")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0x7181932Da75beE6D3604F4ae56077B52fB0c5a3b".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
        let block_number = BlockId::Number(BlockNumber::Number(U64::from(16191695u64)));
        let block = web3.eth().block(block_number).await?;
        println!("Block of {:?}: {:?}", block_number, block);
    }
    Ok(())
}
