use crate::command::handler;
use crate::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Start => handler::handle_start().await,
        Opts::Affirm { command } => handler::handle_affirm(command).await,
    }
}
