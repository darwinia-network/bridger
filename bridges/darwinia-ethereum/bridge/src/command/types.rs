use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge darwinia-goerli options
#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-goerli", about = "Bridge darwinia-goerli")]
pub enum Opts {
    /// Start bridge darwinia-goerli
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
