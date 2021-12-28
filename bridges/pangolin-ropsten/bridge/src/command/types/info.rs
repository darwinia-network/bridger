use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Ecdsa options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub enum InfoOpts {
    /// d2e
    D2e {
        /// d2e command
        #[structopt(flatten)]
        command: D2eCommand,
    },
}

/// D2E command
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct D2eCommand {
    /// Network
    #[structopt(short, long)]
    pub network: String,

    /// tx block
    #[structopt(long)]
    pub txblock: u64,

    /// mmr block
    #[structopt(long)]
    pub mmrblock: u64,

    /// sign block
    #[structopt(long)]
    pub signblock: u64,
}
