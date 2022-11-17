use structopt::StructOpt;

use crate::types::BridgeFlow;

/// Bridge pangolin-pangolinparachainalpha operations
#[derive(Debug, StructOpt)]
#[structopt(
    name = "pangolin-pangolinparachainalpha",
    about = "Bridge pangolin-pangolinparachainalpha"
)]
pub enum Opts {
    /// Init bridge
    Init {
        /// Bridge flow direction, [moonbase-to-pangolin | pangolin-to-pangolinparachainalpha]
        #[structopt()]
        bridge: BridgeFlow,
    },
    /// Start bridge
    Start,
}
