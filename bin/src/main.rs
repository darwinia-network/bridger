use structopt::StructOpt;

use crate::command::types::Opt;

mod cli;
mod command;
mod config;
mod external;
mod initialize;

fn main() -> color_eyre::Result<()> {
    initialize::init()?;

    let ret: Result<Opt, structopt::clap::Error> = Opt::from_args_safe();
    match ret {
        Ok(opt) => cli::execute(opt)?,
        Err(e) => {
            if structopt::clap::ErrorKind::UnknownArgument == e.kind {
                let args_orig: Vec<String> = std::env::args().collect();
                let sub_command = &args_orig[1..2][0];
                let args_sub = &args_orig[2..];
                println!("{} {:?}", sub_command, args_sub);
            }
            e.exit();
        }
    }

    Ok(())
}
