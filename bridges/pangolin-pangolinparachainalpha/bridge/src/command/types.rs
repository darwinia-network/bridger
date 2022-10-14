use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge pangolin-pangolinparachainalpha options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "pangolin-pangolinparachainalpha",
    about = "Bridge pangolin-pangolinparachainalpha"
)]
pub enum Opts {
    /// Init bridge pangolin-pangolinparachainalpha
    Init {
        /// Bridge, support pangolin-to-pangolin-parachain-alpha moonbase-to-pangolin
        #[structopt()]
        bridge: BridgeName,
    },
    Start,
}
