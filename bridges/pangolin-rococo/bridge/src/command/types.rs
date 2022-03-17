use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge pangolin-pangoro options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-pangoro", about = "Bridge pangolin-pangoro")]
pub enum Opts {
    /// Init bridge pangolin-pangoro
    Init {
        /// Bridge, support pangolin-to-pangoro pangoro-to-pangolin
        #[structopt()]
        bridge: BridgeName,
    },
}
