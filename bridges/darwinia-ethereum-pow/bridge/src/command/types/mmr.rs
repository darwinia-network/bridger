use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// MMR options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct MmrOpts {
    /// Network
    #[structopt(short, long)]
    pub network: String,

    /// mmr block
    #[structopt(long)]
    pub mmrblock: u64,
}
