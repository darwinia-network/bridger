use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge darwinia-crab options
#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-crab", about = "Bridge darwinia-crab")]
pub enum Opts {
    /// Start bridge darwinia-crab
    Start,
    /// Init bridge darwinia-crab
    Init {
        /// Bridge, support darwinia-to-crab crab-to-darwinia
        #[structopt()]
        bridge: BridgeName,
    },
}
