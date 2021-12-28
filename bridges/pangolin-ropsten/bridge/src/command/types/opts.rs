use structopt::StructOpt;

use crate::command::types::AffirmOpts;

/// Bridge pangolin-ropsten options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-ropsten", about = "Bridge pangolin-ropsten")]
pub enum Opts {
    /// Start bridge pangolin-ropsten
    Start,
    /// Do affirm
    Affirm {
        /// Commands of affirm
        #[structopt(flatten)]
        command: AffirmOpts,
    },
}
