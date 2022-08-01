use crate::command::types::Opts;
use structopt::StructOpt;

mod bridge;
mod cli;
mod command;
mod kiln_client;
mod message_contract;
mod pangoro_client;
mod service;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
