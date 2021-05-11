use structopt::StructOpt;

use types::Opt;

use crate::error::Result;

mod config;
mod initialize;
mod server;
mod types;

pub async fn exec() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::InitBridge {
            server,
            token,
            source,
            target,
        } => {
            return initialize::exec(server, token, source, target).await;
        }
        Opt::Start {
            config,
            host,
            port,
            enable_auth,
        } => {
            return server::exec(config, host, port, enable_auth).await;
        }
        Opt::Config(config) => {
            return config::exec(config).await;
        }
    }
    Ok(())
}
