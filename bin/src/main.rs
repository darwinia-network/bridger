use structopt::StructOpt;

use crate::types::command::Opt;

mod handler;
mod initialize;
mod patch;
mod route;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize::init()?;
    let opt = Opt::from_args();
    match opt {
        Opt::Server { options } => {
            handler::handle_server(options).await?;
        }
        Opt::Task { server, command } => {
            handler::handle_task(server, command).await?;
        }
        Opt::Kv {
            server,
            namespace,
            command,
        } => handler::handle_kv(server, namespace, command).await?,
    };
    Ok(())
}
