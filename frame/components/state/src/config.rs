use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Microkv config
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MicrokvConfig {
    /// The base path for bridger persist data
    pub base_path: PathBuf,
    /// THe kv database name
    pub db_name: Option<String>,
    /// Kv database is auto commit
    pub auto_commit: bool,
}
