use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Microkv config
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MicrokvConfig {
    pub base_path: PathBuf,
    pub db_name: Option<String>,
    pub auto_commit: bool,
}
