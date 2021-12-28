use structopt::StructOpt;

/// Bridge pangolin-ropsten options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-ropsten", about = "Bridge pangolin-ropsten")]
pub enum Opts {
    /// Start bridge pangolin-ropsten
    Start,
}
