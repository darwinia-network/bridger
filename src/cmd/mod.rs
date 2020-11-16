//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod affirm;
mod affirm_raw;
mod affirmations;
mod confirm;
mod guard;
mod keys;
mod run;
mod show_parcel;
mod set_start;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Run the bridger, this will start `ethereum`, `relay`, `redeem` and `guard` services
    Run {
        /// Data dir of bridger
        #[structopt(short, long)]
        data_dir: Option<PathBuf>,
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
    /// Affirm one target block
    Affirm {
        /// The block number to affirm
        #[structopt(short, long)]
        block: u64,
    },
    /// Affirm a raw parcel from json str
    AffirmRaw {
        /// The block number to affirm
        #[structopt(short, long)]
        json: String,
    },
    /// Show sudo and technical committee members' public key
    Keys,
    /// List affirmations from chain
    Affirmations,
    /// Run `guard` service standalone
    Guard,
    /// Show a parcel from ethereum
    ShowParcel {
        /// The block number to affirm
        #[structopt(short, long)]
        block: u64,
        /// json format
        #[structopt(short, long)]
        json: bool,
    },
    /// Set where to start the ethereum scan
    SetStart {
        /// Data dir of bridger
        #[structopt(short, long)]
        data_dir: Option<PathBuf>,
        /// The new ethereum start
        #[structopt(short, long)]
        block: u64,
    },
}

/// Exec commands
pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run { data_dir, verbose } => run::exec(data_dir, verbose).await,
        Opt::Confirm { block } => confirm::exec(block).await?,
        Opt::Affirm { block } => affirm::exec(block).await?,
        Opt::AffirmRaw { json } => affirm_raw::exec(json).await?,
        Opt::ShowParcel { block, json } => show_parcel::exec(block, json).await?,
        Opt::Keys => keys::exec().await?,
        Opt::Affirmations => affirmations::exec().await?,
        Opt::Guard => guard::exec().await?,
        Opt::SetStart { data_dir, block }  => set_start::exec(data_dir, block).await?,
    }

    Ok(())
}
