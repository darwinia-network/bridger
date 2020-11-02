//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};
use std::time::Duration;
use tokio::time;

mod confirm;
mod run;
mod affirm;
mod keys;
mod affirmations;
mod guard;
mod show_parcel;
mod affirm_raw;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Run the bridger, this will start `ethereum`, `relay`, `redeem` and `guard` services
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
}

/// Exec commands
pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Run { config, verbose } => {
			if std::env::var("RUST_LOG").is_err() {
				if verbose {
					std::env::set_var("RUST_LOG", "info,darwinia_bridger");
				} else {
					std::env::set_var("RUST_LOG", "info");
				}
			}
			env_logger::init();

			while run::exec(config.clone()).await.is_err() {
				time::delay_for(Duration::from_secs(5)).await;
			}
		}
        Opt::Confirm { block } => confirm::exec(block).await?,
        Opt::Affirm { block} => affirm::exec(block).await?,
        Opt::AffirmRaw { json } => affirm_raw::exec(json).await?,
        Opt::ShowParcel { block, json } => show_parcel::exec(block, json).await?,
        Opt::Keys => keys::exec().await?,
        Opt::Affirmations => affirmations::exec().await?,
        Opt::Guard => guard::exec().await?,
    }

    Ok(())
}
