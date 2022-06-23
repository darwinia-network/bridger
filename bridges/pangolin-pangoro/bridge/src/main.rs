use structopt::StructOpt;

use crate::command::types::Opts;

mod bridge;
mod cli;
mod command;
mod service;
mod types;
mod feemarket;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
