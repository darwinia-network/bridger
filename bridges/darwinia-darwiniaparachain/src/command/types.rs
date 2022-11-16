use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge darwinia-darwiniaparachain operations
#[derive(Debug, StructOpt)]
#[structopt(
    name = "darwinia-darwiniaparachain",
    about = "Bridge darwinia-darwiniaparachain"
)]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [polkadot-to-darwinia | darwinia-to-darwinia-parachain]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
