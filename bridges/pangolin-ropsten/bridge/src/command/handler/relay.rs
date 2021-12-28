use component_state::state::BridgeState;
use support_terminal::output;

use crate::bridge::PangolinRopstenTask;
use crate::command::types::RelayOpts;

pub async fn handle_relay(opts: RelayOpts) -> color_eyre::Result<()> {
    let block = opts.block;

    let state = BridgeState::new()?;
    let microkv = state.microkv_with_namespace(PangolinRopstenTask::name());

    let target = microkv.get_as("affirm.target")?.unwrap_or(0);

    if block > target {
        microkv.put("affirm.target", &block)?;
        output::output_warning("You need run `start` to start bridge");
        output::output_text("Success");
    } else {
        output::output_warning(format!(
            "The relay block ({}) is less than current block ({})",
            block, target
        ));
    }
    Ok(())
}
