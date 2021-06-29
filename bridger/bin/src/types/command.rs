use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "darwinia-bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Task manager
    Task(TaskCommand),
}

#[derive(Debug, StructOpt)]
pub enum TaskCommand {
    /// List of available task
    List,
    /// Start a task
    Start,
    /// Stop a running task
    Stop,
}
