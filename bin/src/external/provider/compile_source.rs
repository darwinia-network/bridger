use colored::Colorize;

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
        println!("Compile");
        Ok(())
    }
}
