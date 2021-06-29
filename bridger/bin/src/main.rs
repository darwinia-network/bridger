#[macro_use]
extern crate log;

use structopt::StructOpt;

use crate::types::command::Opt;

mod handler;
mod types;

fn init() {
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    self::init();
    let opt = Opt::from_args();
    match opt {
        Opt::Task(command) => {
            handler::handle_task_command(command)?;
        }
    };
    debug!("Bridge started!");
    Ok(())
}
