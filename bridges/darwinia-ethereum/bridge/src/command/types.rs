use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge darwinia-ethereum options
#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-ethereum", about = "Bridge darwinia-ethereum")]
pub enum Opts {
    /// Start bridge darwinia-ethereum
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
