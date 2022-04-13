use structopt::StructOpt;

use crate::command::types::Opts;

mod bridge;
mod chains;
mod cli;
mod command;
mod service;
mod traits;
mod types;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
