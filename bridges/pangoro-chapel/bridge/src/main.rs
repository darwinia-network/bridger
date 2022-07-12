use structopt::StructOpt;

use crate::command::types::Opts;

mod bridge;
mod cli;
mod command;
mod pangoro_client;
mod service;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
