use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge darwinia-crab operations
#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-crab", about = "Bridge darwinia-crab")]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [darwinia-to-crab | crab-to-darwinia]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
