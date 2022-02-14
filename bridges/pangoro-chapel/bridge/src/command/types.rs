use structopt::StructOpt;

/// Bridge template options
#[derive(Debug, StructOpt)]
#[structopt(name = "bridge-template", about = "Bridge template")]
pub enum Opts {
    /// Start bridge template
    Start,
}
