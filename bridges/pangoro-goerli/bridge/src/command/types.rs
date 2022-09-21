use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge pangoro-goerli options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangoro-goerli", about = "Bridge pangoro-goerli")]
pub enum Opts {
    /// Start bridge pangoro-goerli
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
