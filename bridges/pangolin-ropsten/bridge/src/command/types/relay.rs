use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Relay options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct RelayOpts {
    /// block
    #[structopt(long)]
    pub block: u64,
}
