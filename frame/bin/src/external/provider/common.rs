use std::path::PathBuf;

use cargo_util::ProcessBuilder;
use colored::Colorize;

use support_common::error::BridgerError;

/// Execute binary
pub fn execute_binary(
    command: String,
    path_binary: PathBuf,
    args: Vec<String>,
    cwd: PathBuf,
) -> color_eyre::Result<()> {
    let mut builder_bridge = ProcessBuilder::new(path_binary);
    builder_bridge.args(args.as_slice()).cwd(&cwd);
    for (n, v) in std::env::vars() {
        builder_bridge.env(&n, v);
    }

    tracing::info!(
        "Execute `{} {}` in path: {}",
        &command.green(),
        args.join(" ").green(),
        cwd.display()
    );
    if let Err(e) = builder_bridge.exec() {
        return Err(BridgerError::Process(command, args.join(" "), format!("{:?}", e)).into());
    }
    Ok(())
}
