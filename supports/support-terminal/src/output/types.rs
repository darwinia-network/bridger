use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(
    Clone, Debug, Deserialize, Serialize, StructOpt, strum::EnumString, strum::EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum OutputFormat {
    Raw,
    Json,
    Table,
}
