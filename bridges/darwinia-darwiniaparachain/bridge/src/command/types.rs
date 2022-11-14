use structopt::StructOpt;

use crate::types::BridgeName;

// Bridge darwinia-darwiniaparachain options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "darwinia-darwiniaparachain",
    about = "Bridge darwinia-darwiniaparachain"
)]
pub enum Opts {
    /// Init bridge darwinia-darwiniaparachain
    Init {
        /// Bridge
        #[structopt()]
        bridge: BridgeName,
    },
    Start,
}
