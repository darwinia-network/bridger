use structopt::StructOpt;
use support_terminal::types::BasicOptions;

/// Bridge template options
#[derive(Debug, StructOpt)]
#[structopt(name = "bridge-template", about = "Bridge template")]
pub enum Opts {
    /// Start bridge template
    Start {
        /// basic options
        #[structopt(flatten)]
        basic_options: BasicOptions,
    },
}
