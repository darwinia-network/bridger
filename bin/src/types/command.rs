use std::path::PathBuf;

use structopt::StructOpt;

use bridge_traits::bridge::config::ConfigFormat;

#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Task manager
    Task {
        /// The server host by darwinia-bridger service
        #[structopt(long, default_value = "http://127.0.0.1:1098")]
        server: String,
        #[structopt(flatten)]
        command: TaskCommand,
    },
    /// The bridge kv db storage operation
    Kv {
        /// The server host by darwinia-bridger service
        #[structopt(long, default_value = "http://127.0.0.1:1098")]
        server: String,
        /// The namespace of storage
        #[structopt(long, short)]
        namespace: Option<String>,
        #[structopt(flatten)]
        command: KvCommand,
    },
    /// Start bridger server
    Server {
        #[structopt(flatten)]
        options: ServerOptions,
    },
}

#[derive(Debug, StructOpt)]
pub enum KvCommand {
    /// Put Key-Value to bridger database
    Put {
        /// keys and values one by one
        #[structopt()]
        kvs: Vec<String>,
    },
    /// Get Key-Value from bridger
    Get {
        /// Get a value by key
        #[structopt()]
        keys: Vec<String>,
    },
    /// List bridger database
    List {
        /// List by sorted
        #[structopt(short, long)]
        sorted: bool,
    },
    /// Remove a Key-Value from bridger
    Remove {
        /// Remove a value by key
        #[structopt()]
        keys: Vec<String>,
    },
}

#[derive(Debug, StructOpt)]
pub enum TaskCommand {
    /// List of available task
    List,
    /// Start a task
    Start {
        /// Options of task control
        #[structopt(flatten)]
        options: TaskControlOptions,
    },
    /// Restart a task
    Restart {
        /// Options of task control
        #[structopt(flatten)]
        options: TaskControlOptions,
    },
    /// Stop a running task
    Stop {
        /// The task name
        #[structopt(short, long)]
        name: String,
    },
    /// Execute task command
    Exec {
        /// Options of task execute
        #[structopt(flatten)]
        options: TaskExecuteOptions,
    },
    /// Show config template
    ConfigTemplate {
        /// The task name
        #[structopt(short, long)]
        name: String,
        /// The config format, supports [toml|json|yml]
        #[structopt(long, default_value = "toml")]
        format: ConfigFormat,
    },
}

#[derive(Clone, Debug, StructOpt)]
pub struct TaskExecuteOptions {
    /// The task name
    #[structopt(short, long)]
    pub name: String,
    /// The api of task
    #[structopt(short, long)]
    pub api: String,
    /// The parameters of this api
    #[structopt(short, long, default_value = "")]
    pub param: Vec<String>,
}

#[derive(Clone, Debug, StructOpt)]
pub struct TaskControlOptions {
    /// The task name
    #[structopt(short, long)]
    pub name: String,
    /// The config format, supports [toml|json|yml]
    #[structopt(long, default_value = "toml")]
    pub format: ConfigFormat,
    /// The config file path, When first run this is required, but the server already have this task config, can be skip this parameter
    #[structopt(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Clone, Debug, StructOpt)]
pub struct ServerOptions {
    /// Bridger service listen host
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,
    /// Bridger service listen port
    #[structopt(short, long, default_value = "1098")]
    pub port: u32,
    /// The bridger config or data base path.
    #[structopt(long, parse(from_os_str))]
    pub base_path: Option<PathBuf>,
}
