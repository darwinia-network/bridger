use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge pangolin-pangoro operations
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-pangoro", about = "Bridge pangolin-pangoro")]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [pangolin-to-pangoro | pangoro-to-pangolin]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
