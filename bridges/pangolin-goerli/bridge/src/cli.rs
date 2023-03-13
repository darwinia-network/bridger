use crate::command::handler;
use crate::command::types::Opts;
use component_state::state::StateOptions;
use subquery::types::BridgeName;

/// Execute command
pub async fn execute(opts: Opts) -> color_eyre::Result<()> {
    match opts {
        Opts::Start => handler::handle_start().await,
        Opts::Kv { command } => {
            let task_name = BridgeName::PangolinGoerli.name();
            let namespace = command.namespace.unwrap_or_else(|| task_name.to_string());
            let state_options = StateOptions {
                db_name: task_name.to_string(),
            };
            support_command_kv::handle_kv(state_options, Some(namespace), command.command)
        }
    }
}
