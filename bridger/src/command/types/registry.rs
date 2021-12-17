use strum::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
pub enum RegistryType {
    Local,
    Github,
    Server,
}
