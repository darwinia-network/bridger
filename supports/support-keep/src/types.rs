use std::path::PathBuf;
use std::sync::Arc;

use bridge_traits::bridge::config::ConfigFormat;

/// The webserver state struct
#[derive(Clone, Debug)]
pub struct WebserverState {
    /// Base path of bridger web server, It's a folder
    pub base_path: Arc<PathBuf>,
}

/// The task state struct
#[derive(Clone, Debug)]
pub struct TaskState {
    /// The task config path, It's a file
    pub config_path: PathBuf,
    /// The task config format, supports [toml|json|yml]
    pub config_format: ConfigFormat,
}
