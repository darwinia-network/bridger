//! Sup Commands
use crate::error::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

mod affirm;
mod affirm_raw;
mod affirmations;
mod confirm;
mod ecdsa;
mod guard;
mod keys;
mod run;
mod set_darwinia_start;
mod set_start;
mod show_parcel;
mod info_darwinia2ethereum;

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
	/// Ecdsa sign and send
	Ecdsa {
		/// The new ethereum start
		#[structopt(short, long)]
		message: String,
	},
	/// Set where to start the darwinia scan
	SetDarwiniaStart {
		/// Data dir of bridger
		#[structopt(short, long)]
		data_dir: Option<PathBuf>,
		/// The new ethereum start
		#[structopt(short, long)]
		block: u64,
	},
    /// Get Darwinia to Ethereum proof info
    InfoD2E {
        /// tx block number
        txblock: u64,
        /// mmr block number
        mmrblock: u64,
        /// sign block number
        signblock: u64,
    }
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
		Opt::SetStart { data_dir, block } => set_start::exec(data_dir, block).await?,
		Opt::Ecdsa { message } => ecdsa::exec(message).await?,
		Opt::SetDarwiniaStart { data_dir, block } => {
			set_darwinia_start::exec(data_dir, block).await?
		},
        Opt::InfoD2E { txblock, mmrblock, signblock } => {
            info_darwinia2ethereum::exec( txblock, mmrblock, signblock ).await?
        },
	}

	Ok(())
}
