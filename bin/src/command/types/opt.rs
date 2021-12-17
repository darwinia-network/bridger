use structopt::StructOpt;

use crate::command::types::RegistryType;

#[derive(Debug, StructOpt)]
#[structopt(name = "bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Bridger registroy,
    Registry {
        /// Registry type, support local|github|server, default is github
        #[structopt(long = "type", default_value = "local")]
        type_: RegistryType,
        /// The path of registry
        #[structopt(long)]
        path: Option<String>,
    },
    /// List all bridges
    List,
}
