use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Ecdsa options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct EcdsaOpts {
    /// ecdsa message
    #[structopt(short, long)]
    pub message: String,
}
