use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge pangolin-crabparachain options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-crabparachain", about = "Bridge pangolin-crabparachain")]
pub enum Opts {
    /// Init bridge pangolin-crabparachain
    Init {
        /// Bridge, support rococo-to-pangolin
        #[structopt()]
        bridge: BridgeName,
    },
    Start,
}
