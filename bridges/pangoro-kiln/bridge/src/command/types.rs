use structopt::StructOpt;
use support_command_kv::NamespaceKvOpts;

/// Bridge pangoro-kiln options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangoro-kiln", about = "Bridge pangoro-kiln")]
pub enum Opts {
    /// Start bridge pangoro-kiln
    Start,
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
