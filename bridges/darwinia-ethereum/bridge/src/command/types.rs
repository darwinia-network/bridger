use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge darwinia-eth options
#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-eth", about = "Bridge darwinia-eth")]
pub enum Opts {
    /// Start bridge darwinia-eth
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
