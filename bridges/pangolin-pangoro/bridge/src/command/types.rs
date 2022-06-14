use structopt::StructOpt;

use crate::types::BridgeName;

/// Bridge template options
#[derive(Debug, StructOpt)]
#[structopt(name = "bridge-template", about = "Bridge template")]
pub enum Opts {
    /// Init bridge pangolin-pangoro
    Init {
        /// Bridge, support pangolin-to-pangoro pangoro-to-pangolin
        #[structopt()]
        bridge: BridgeName,
    },
    /// Start bridge template
    Start,
}
