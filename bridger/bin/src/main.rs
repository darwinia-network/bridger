use structopt::StructOpt;

use crate::types::command::Opt;

mod dc;
mod handler;
mod types;

fn init() -> anyhow::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        r#"
        serde=info,
        lifeline=debug,
        darwinia_bridge=debug,
        bridge_shared=debug,
        shared-darwinia=debug,
        service_darwinia_ethereum=debug,
        task-darwinia-ethereum=debug,
        "#,
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::init()?;
    let opt = Opt::from_args();
    match opt {
        Opt::Server { options } => {
            handler::handle_server(options).await?;
        }
        Opt::Task(command) => {
            handler::handle_task(command).await?;
        }
        Opt::Shared(command) => {
            handler::handle_shared(command).await?;
        }
    };
    Ok(())
}
