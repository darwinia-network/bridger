use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge pangolin-goerli options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-goerli", about = "Bridge pangolin-goerli")]
pub enum Opts {
    /// Start bridge pangolin-goerli
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
