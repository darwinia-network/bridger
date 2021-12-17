use structopt::StructOpt;

use crate::command::types::Opt;

mod cli;
mod command;
mod initialize;

fn main() -> color_eyre::Result<()> {
    initialize::init()?;
    let opt = Opt::from_args();
    cli::execute(opt)
}
