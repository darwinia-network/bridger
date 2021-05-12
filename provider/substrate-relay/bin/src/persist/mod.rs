use std::path::PathBuf;

use getset::{Getters, MutGetters, Setters};
use typed_builder::TypedBuilder;

use crate::error;

mod chain;
mod persist;

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, MutGetters, Getters, Setters,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Persist {
    #[serde(default)]
    generic: Generic,
    #[serde(default)]
    chains: Vec<Chain>,
    #[serde(default)]
    tokens: Vec<Token>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, MutGetters, Getters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Generic {
    #[serde(skip_deserializing)]
    config_file: PathBuf,
    #[serde(default)]
    host: String,
    #[serde(default)]
    port: u32,
    #[serde(default)]
    enable_auth: bool,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, MutGetters, Getters, Setters,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Chain {
    #[serde(default)]
    name: String,
    #[serde(default)]
    host: String,
    #[serde(default)]
    port: u32,
    #[serde(default)]
    signer: String,
    #[serde(default)]
    secure: bool,
    signer_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Token {
    #[serde(default)]
    value: String,
    remark: Option<String>,
}

impl Default for Generic {
    fn default() -> Self {
        Self {
            config_file: Default::default(),
            host: "127.0.0.1".to_string(),
            port: 7890,
            enable_auth: false,
        }
    }
}
