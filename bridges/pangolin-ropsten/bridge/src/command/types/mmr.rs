use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// MMR options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct MmrOpts {
    /// mmr block
    #[structopt(long)]
    pub mmrblock: u64,
}
