use std::path::PathBuf;
use structopt::StructOpt;

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
    /// Bridge shared service
    Shared {
        /// The server host by darwinia-bridger service
        #[structopt(long, default_value = "http://127.0.0.1:1098")]
        server: String,
        #[structopt(flatten)]
        command: SharedCommand,
    },
    /// Start bridger server
    Server {
        #[structopt(flatten)]
        options: ServerOptions,
    },
}

#[derive(Debug, StructOpt)]
pub enum TaskCommand {
    /// List of available task
    List,
    /// Start a task
    Start {
        /// The task name
        #[structopt(short, long)]
        name: String,
        /// The config format, supports [toml|json|yml]
        #[structopt(long, default_value = "toml")]
        format: String,
        /// The config file path, When first run this is required, but the server already have this task config, can be skip this parameter
        #[structopt(short, long)]
        config: Option<PathBuf>,
    },
    /// Stop a running task
    Stop {
        /// The task name
        #[structopt(short, long)]
        name: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum SharedCommand {
    /// Start shared service
    Start {
        /// The config format, supports [toml|json|yml]
        #[structopt(long, default_value = "toml")]
        format: String,
        #[structopt(short, long, parse(from_os_str))]
        config: Option<PathBuf>,
    },
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
