use serde::{Deserialize, Serialize};

use bridge_traits::bridge::config::ConfigFormat;

fn default_config_format() -> ConfigFormat {
    ConfigFormat::Toml
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SharedStartParam {
    #[serde(default = "default_config_format")]
    pub format: ConfigFormat,
    pub config: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskStartParam {
    #[serde(default = "default_config_format")]
    pub format: ConfigFormat,
    pub name: String,
    pub config: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskStopParam {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub name: String,
    pub running: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskConfigTemplateParam {
    pub name: String,
    #[serde(default = "default_config_format")]
    pub format: ConfigFormat,
}
