use structopt::StructOpt;

/// Bridge pangoro-chapel options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangoro-chapel", about = "Bridge pangoro-chapel")]
pub enum Opts {
    /// Start bridge pangoro-chapel
    Start,
}
