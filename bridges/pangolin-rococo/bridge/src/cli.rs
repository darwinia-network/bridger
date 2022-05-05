use crate::command::handler;
use crate::command::types::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Init { bridge } => handler::handle_init(bridge).await,
        Opts::Relay => handler::handle_relay().await,
    }
}
