use crate::command::handler;
use crate::error::BinS2SResult;

/// Execute command
pub async fn execute(opts: Opts) -> BinS2SResult<()> {
    match opts {
        Opts::Init { bridge } => handler::handle_init(bridge).await,
        Opts::Start => handler::handle_relay().await,
    }
}
