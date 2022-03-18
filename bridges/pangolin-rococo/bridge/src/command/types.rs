use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge pangolin-rococo options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-rococo", about = "Bridge pangolin-rococo")]
pub enum Opts {
    /// Init bridge pangolin-rococo
    Init {
        /// Bridge, support rococo-to-pangolin
        #[structopt()]
        bridge: BridgeName,
    },
}
