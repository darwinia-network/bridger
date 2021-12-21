use cargo_util::ProcessBuilder;
use colored::Colorize;

use support_common::error::BridgerError;

use crate::external::execute::ISubcommandExecutor;
use crate::external::types::CompileChannel;

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
        self.compile_bridge()?;
        Ok(())
    }
}

impl CompileSourceExecutor {
    fn compile_bridge(&self) -> color_eyre::Result<()> {
        let path_crate = std::env::current_dir()?;
        let path_bridge = path_crate.join("bridges").join(&self.command);
        if !path_bridge.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "The bridge [{}] folder not exists. full path is: {:?}",
                    &self.command, &path_bridge
                ),
            )
            .into());
        }

        tracing::info!(
            "Try compile {} in path: {}",
            &self.command.blue(),
            path_bridge.display()
        );
        let mut args = Vec::<String>::new();
        args.push("build".to_string());
        if self.channel == CompileChannel::Release {
            let name = format!("--{}", self.channel.name());
            args.push(name);
        }
        args.push("-p".to_string());
        args.push(self.command.clone());
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
                    format!("{}.exe", &self.command)
                } else {
                    self.command.clone()
                });

        let mut builder_bridge = ProcessBuilder::new(path_binary);
        builder_bridge.args(self.args.as_slice()).cwd(&path_bridge);
        for (n, v) in std::env::vars() {
            builder_bridge.env(&n, v);
        }

        tracing::info!(
            "Execute `{} {}` in path: {}",
            &self.command.green(),
            self.args.join(" ").green(),
            path_bridge.display()
        );
        if let Err(e) = builder_bridge.exec() {
            return Err(BridgerError::Process(
                self.command.clone(),
                self.args.join(" "),
                format!("{:?}", e),
            )
            .into());
        }
        Ok(())
    }
}
