use structopt::StructOpt;

/// Bridge pangoro-kiln options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangoro-kiln", about = "Bridge pangoro-kiln")]
pub enum Opts {
    /// Start bridge pangoro-kiln
    Start,
}
