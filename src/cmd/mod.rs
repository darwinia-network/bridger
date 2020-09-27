//! Sup Commands
use crate::result::Result;
use structopt::{clap::AppSettings, StructOpt};

mod run;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Run the bridger
    Run,
}

/// Exec commands
pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run => run::exec().await?,
    }

    Ok(())
}
