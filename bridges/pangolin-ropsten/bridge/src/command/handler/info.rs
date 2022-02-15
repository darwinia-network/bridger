use support_terminal::output;

use crate::command::types::{D2eCommand, InfoOpts};

pub async fn handle_info(opts: InfoOpts) -> color_eyre::Result<()> {
    match opts {
        InfoOpts::D2e { command } => handle_d2e(command).await,
    }
}

async fn handle_d2e(_command: D2eCommand) -> color_eyre::Result<()> {
    // let bridge_config: PangolinRopstenConfig = Config::restore(Names::BridgePangolinRopsten)?;
    // let network = command.network;
    // let txblock = command.txblock;
    // let mmrblock = command.mmrblock;
    // let signblock = command.signblock;

    output::output_text("Not support now");
    Ok(())
}
