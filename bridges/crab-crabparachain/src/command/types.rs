use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge crab-crabparachain operations
#[derive(Debug, StructOpt)]
#[structopt(name = "crab-crabparachain", about = "Bridge crab-crabparachain")]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [kusama-to-crab | crab-to-crabparachain]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
