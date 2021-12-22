use cargo_util::ProcessBuilder;
use colored::Colorize;
use std::path::PathBuf;

use support_common::error::BridgerError;

use crate::external;
use crate::external::execute::ISubcommandExecutor;
use crate::external::types::CompileChannel;

/// Compile source code and execute binary
#[derive(Clone, Debug)]
pub struct CompileSourceExecutor {
    command: String,
    args: Vec<String>,
    channel: CompileChannel,
}

impl CompileSourceExecutor {
    pub fn new(command: String, args: Vec<String>, channel: CompileChannel) -> Self {
        Self {
            command,
            args,
            channel,
        }
    }
}

impl ISubcommandExecutor for CompileSourceExecutor {
    fn execute(&self, _path: Option<String>) -> color_eyre::Result<()> {
        self.try_compile_and_execute()?;
        Ok(())
    }
}

impl CompileSourceExecutor {
    fn try_compile_and_execute(&self) -> color_eyre::Result<()> {
        let path_crate = std::env::current_dir()?;

        let mut exists = false;
        for prefix in support_common::constants::ALLOW_BINARY_PREFIX {
            let command = format!("{}{}", prefix, self.command);
            let path_bridge = path_crate.join("bridges").join(&command);
            if !path_bridge.exists() {
                continue;
            }
            exists = true;
            self.try_compile_and_execute_with_command(path_bridge, command)?;
            break;
        }
        if !exists {
            return Err(BridgerError::UnsupportExternal(format!(
                "Not support this subcommand: {}",
                self.command
            ))
            .into());
        }
        Ok(())
    }

    fn try_compile_and_execute_with_command(
        &self,
        path_bridge: PathBuf,
        command: impl AsRef<str>,
    ) -> color_eyre::Result<()> {
        let command = command.as_ref();
        tracing::info!(
            "Try compile {} in path: {}",
            &command.blue(),
            path_bridge.display()
        );
        let mut args = Vec::<String>::new();
        args.push("build".to_string());
        if self.channel == CompileChannel::Release {
            let name = format!("--{}", self.channel.name());
            args.push(name);
        }
        args.push("-p".to_string());
        args.push(command.to_string());
        let args = args.as_slice();

        let mut builder_cargo = ProcessBuilder::new("cargo");
        builder_cargo.args(args).cwd(&path_bridge);

        tracing::info!(
            "Execute `{} {}` in path: {}",
            "cargo".green(),
            args.join(" ").green(),
            path_bridge.display()
        );
        if let Err(e) = builder_cargo.exec() {
            return Err(BridgerError::Process(
                "cargo".to_string(),
                args.join(" "),
                format!("{:?}", e),
            )
            .into());
        }

        // when compiled success, prepare execute this binary

        let path_binary =
            path_bridge
                .join("target")
                .join(self.channel.name())
                .join(if cfg!(windows) {
                    format!("{}.exe", &command)
                } else {
                    command.to_string()
                });

        external::provider::common::execute_binary(
            command.to_string(),
            path_binary,
            self.args.clone(),
            path_bridge,
        )
    }
}
