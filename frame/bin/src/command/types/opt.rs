use structopt::StructOpt;

use crate::command::types::RegistryOpt;

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
}
