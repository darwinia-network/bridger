use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTaskWrapper {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub commands: Vec<TaskCommand>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskCommand {
    pub name: String,
    pub path: Option<String>,
    #[serde(default)]
    pub headers: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub bodies: Vec<CommandBody>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandBody {
    pub name: String,
    pub short: Option<String>,
    pub long: Option<String>,
    pub types: String,
    pub description: Option<String>,
    pub default: Option<String>,
    #[serde(default)]
    pub derives: Vec<String>,
}
