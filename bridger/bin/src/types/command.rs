use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Task manager
    Task(TaskCommand),
    /// Bridge shared service
    Shared(SharedCommand),
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
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        config: String,
    },
    /// Stop a running task
    Stop {
        #[structopt(short, long)]
        name: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum SharedCommand {
    /// Start shared service
    Start {
        #[structopt(short, long)]
        config: String,
    },
}

#[derive(Debug, StructOpt)]
pub struct ServerOptions {
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,
    #[structopt(short, long, default_value = "1098")]
    pub port: u32,
}
