use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge pangolin-pangolinparachain options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "pangolin-pangolinparachain",
    about = "Bridge pangolin-pangolinparachain"
)]
pub enum Opts {
    /// Init bridge pangolin-pangolinparachain
    Init {
        /// Bridge
        #[structopt()]
        bridge: BridgeName,
    },
    Start,
}
