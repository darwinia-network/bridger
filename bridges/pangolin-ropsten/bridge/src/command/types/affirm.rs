use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// affirm command mode
#[derive(
    Clone, Debug, Deserialize, Eq, PartialEq, Serialize, strum::EnumString, strum::EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum AffirmMode {
    /// From block number
    Block,
    /// From raw json data
    Raw,
}

/// Affirm options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub enum AffirmOpts {
    /// Do affirm
    Do {
        /// The mode of parameter
        #[structopt(long, default_value = "block")]
        mode: AffirmMode,
        /// The block number for ethereum
        #[structopt(long)]
        block: Option<u64>,
        /// Raw parcel json data
        #[structopt(long = "raw")]
        raw_json: Option<String>,
    },
    /// Show affirm state
    State,
}
