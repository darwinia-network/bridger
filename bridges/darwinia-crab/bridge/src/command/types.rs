use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge darwinia-crab options
#[derive(Debug, StructOpt)]
#[structopt(name = "bridge-darwinia-crab", about = "Bridge darwinia-crab")]
pub enum Opts {
    /// Init bridge darwinia-crab
    Init {
        /// Bridge, support darwinia-to-crab crab-to-darwinia
        #[structopt()]
        bridge: BridgeName,
    },
    /// Start bridge darwinia-crab
    Start,
}
