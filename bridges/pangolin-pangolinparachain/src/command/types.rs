use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge pangolin-pangolinparachain operations
#[derive(Debug, StructOpt)]
#[structopt(
    name = "pangolin-pangolinparachain",
    about = "Bridge pangolin-pangolinparachain"
)]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [rococo-to-pangolin | pangolin-to-pangolin-parachain]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
