use structopt::StructOpt;

/// Bridge pangolin-pangoro options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-pangoro", about = "Bridge pangolin-pangoro")]
pub enum Opts {
    /// Start bridge pangolin-pangoro
    Start,
    /// Init Bridge pangolin-pangoro
    Init,
}
