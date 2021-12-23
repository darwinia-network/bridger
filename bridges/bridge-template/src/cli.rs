use crate::command::handler;
use crate::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Start { basic_options } => handler::handle_start(basic_options).await,
    }
}
