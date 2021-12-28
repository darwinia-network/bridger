use crate::command::handler;
use crate::Opts;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Start => handler::handle_start().await,
        Opts::Affirm { command } => handler::handle_affirm(command).await,
        Opts::Confirm { command } => handler::handle_confirm(command).await,
        Opts::Ecdsa { command } => handler::handle_ecdsa(command).await,
        Opts::Guard => handler::handle_guard().await,
        Opts::Info { command } => handler::handle_info(command).await,
        Opts::Keys => handler::handle_keys().await,
        Opts::Mmr { command } => handler::handle_mmr(command).await,
        Opts::Parcel { command, output } => handler::handle_parcel(command, output).await,
        Opts::Relay { command } => handler::handle_relay(command).await,
    }
}
