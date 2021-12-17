use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum RegistryType {
    Local,
    Github,
    Server,
}
