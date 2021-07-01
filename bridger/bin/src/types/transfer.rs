use serde::{Deserialize, Serialize};

fn default_config_format() -> String {
    "toml".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SharedStartParam {
    #[serde(default = "default_config_format")]
    pub format: String,
    pub config: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskStartParam {
    #[serde(default = "default_config_format")]
    pub format: String,
    pub name: String,
    pub config: Option<String>,
}
