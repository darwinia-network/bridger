use structopt::StructOpt;

use crate::command::types::Opts;

mod bridge;
mod cli;
mod command;
mod service;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
