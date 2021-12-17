use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bridger", about = "Darwinia bridger")]
pub enum Opt {
    /// Bridger registroy,
    Registry,
    /// List all commands
    List,
}
