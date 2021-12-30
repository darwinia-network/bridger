use crate::command::handler;
use crate::command::types::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Start => handler::handle_relay().await,
        Opts::Init => handler::handle_init().await,
    }
}
