use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Parcel options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct ParcelOpts {
    /// block
    #[structopt(long)]
    pub block: u64,
}
