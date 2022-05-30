use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge crab-crabparachain options
#[derive(Debug, StructOpt)]
#[structopt(name = "crab-crabparachain", about = "Bridge crab-crabparachain")]
pub enum Opts {
    /// Init bridge crab-crabparachain
    Init {
        /// Bridge, support kusama-to-crab
        #[structopt()]
        bridge: BridgeName,
    },
    Start,
}
