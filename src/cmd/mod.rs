//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod confirm;
mod run;
mod affirm;
mod keys;
mod affirmations;
mod guard;

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
    /// Set Confirmed block with sudo privilege
    Confirm {
        /// The confirmed block number
        #[structopt(short, long)]
        block: u64,
    },
    /// Affirm a target block
    Affirm {
        /// The block number to affirm
        #[structopt(short, long)]
        block: u64,
    },
    /// Show sudo and technical committee members
    Keys,
    /// List affirmations from chain
    Affirmations,
    /// Run guard standalone
    Guard,
}

/// Exec commands
pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run { config, verbose } => run::exec(config, verbose).await?,
        Opt::Confirm { block } => confirm::exec(block).await?,
        Opt::Affirm { block} => affirm::exec(block).await?,
        Opt::Keys => keys::exec().await?,
        Opt::Affirmations => affirmations::exec().await?,
        Opt::Guard => guard::exec().await?,
    }

    Ok(())
}
