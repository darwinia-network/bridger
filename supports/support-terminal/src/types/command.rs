use std::path::PathBuf;

use structopt::StructOpt;

/// Basic command options
#[derive(Clone, Debug, StructOpt)]
pub struct BasicOptions {
    /// The bridger config or data base path.
    #[structopt(short = "p", long, parse(from_os_str))]
    pub base_path: Option<PathBuf>,
    /// The config file name
    #[structopt(short = "c", long)]
    pub file_name: Option<String>,
}
