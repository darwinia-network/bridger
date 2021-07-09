use structopt::StructOpt;

use crate::types::command::Opt;

mod handler;
mod patch;
mod types;

fn init() -> anyhow::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        r#"
        serde=info,
        lifeline=debug,
        darwinia_bridge=debug,
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
        Opt::Task { server, command } => {
            handler::handle_task(server, command).await?;
        }
    };
    Ok(())
}
