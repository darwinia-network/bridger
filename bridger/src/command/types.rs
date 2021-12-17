use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// List all commands
    List,
}
