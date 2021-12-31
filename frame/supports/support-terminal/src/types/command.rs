use structopt::StructOpt;

use crate::output::OutputFormat;

/// Basic command options
#[derive(Clone, Debug, StructOpt)]
pub struct BasicOptions {
    /// output format
    #[structopt(short, long, default_value = "raw")]
    pub output: OutputFormat,
}
