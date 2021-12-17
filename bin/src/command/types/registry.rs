use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Deserialize, Eq, PartialEq, Serialize, strum::EnumString, strum::EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum RegistryType {
    Local,
    Github,
    Server,
}
