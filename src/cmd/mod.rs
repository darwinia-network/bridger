//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod run;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Run the bridger
    Run {
        /// Config path of bridger
        #[structopt(short, long)]
        config: Option<PathBuf>,
        /// Run bridger in verbose mode
        #[structopt(short, long)]
        verbose: bool,
    },
}

/// Exec commands
pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run { config, verbose } => run::exec(config, verbose).await?,
    }

    Ok(())
}
