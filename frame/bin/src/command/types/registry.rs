use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use support_terminal::output::OutputFormat;

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
        /// Registry type, support local|github|server
        #[structopt(long = "type", default_value = "github")]
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
    /// Set registry version, will be download by your set value
    Version {
        /// The version
        #[structopt()]
        value: Option<String>,
        /// Set version to bridger bundle, if true, not use the value parameter
        #[structopt(long)]
        bundle: bool,
    },
}
