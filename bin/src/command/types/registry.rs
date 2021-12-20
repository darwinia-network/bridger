use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::command::types::OutputFormat;

#[derive(
    Clone, Debug, Deserialize, Eq, PartialEq, Serialize, strum::EnumString, strum::EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum RegistryType {
    Local,
    Github,
    Server,
}

/// Registry option
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub enum RegistryOpt {
    /// Set registry
    Set {
        /// Registry type, support local|github|server, default is github
        #[structopt(long = "type", default_value = "local")]
        type_: RegistryType,
        /// The path of registry
        #[structopt(long)]
        path: Option<String>,
    },
    /// Get current registry
    Get {
        /// The output format
        #[structopt(short, long, default_value = "raw")]
        output: OutputFormat,
    },
}
