use bridge_pangoro_goerli::cli;
use bridge_pangoro_goerli::command::types::Opts;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    support_common::initialize::init()?;
    let opt = Opts::from_args();
    cli::execute(opt).await?;
    Ok(())
}
