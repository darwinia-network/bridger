use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Confirm options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct ConfirmOpts {
    /// The block number for ethereum
    #[structopt(long)]
    pub block: u32,
}
