use structopt::StructOpt;

use crate::command::types::{KvOpt, RegistryOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Bridger registry,
    Registry {
        /// Commands of registry
        #[structopt(flatten)]
        command: RegistryOpt,
    },
    /// List all bridges
    List,
    /// Kv database
    Kv {
        /// The namespace of storage
        #[structopt(long, short)]
        namespace: Option<String>,
        /// Commands of registry
        #[structopt(flatten)]
        command: KvOpt,
    },
}
