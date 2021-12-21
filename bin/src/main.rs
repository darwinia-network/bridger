use structopt::StructOpt;
use support_common::error::BridgerError;

use crate::command::types::Opt;
use crate::external::execute::ExternalExecutor;

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
                let sub_args = &args_orig[2..];
                let executor = ExternalExecutor::new(sub_command.clone(), sub_args.to_vec());
                if let Err(ez) = executor.execute() {
                    if let Some(BridgerError::UnsupportExternal(_msg)) = ez.downcast_ref() {
                        e.exit();
                    }
                    return Err(ez);
                }
                return Ok(());
            }
            e.exit();
        }
    }

    Ok(())
}
