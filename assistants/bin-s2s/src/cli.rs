use crate::command::handler;
use crate::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Init { bridge } => handler::handle_init(bridge).await,
        Opts::Start => handler::handle_relay().await,
    }
}
