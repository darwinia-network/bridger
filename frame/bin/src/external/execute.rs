use support_common::config::{Config, Names};

use crate::command::types::RegistryType;
use crate::config::BridgerConfig;
use crate::external::provider::{CompileSourceExecutor, PrecompiledBinaryExecutor};
use crate::external::types::CompileChannel;

/// External subcommand executor
#[derive(Clone, Debug)]
pub struct ExternalExecutor {
    command: String,
    args: Vec<String>,
}

impl ExternalExecutor {
    /// Create instance of ExternalExecutor
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

impl ExternalExecutor {
    /// Execute external subcommand
    pub fn execute(&self) -> color_eyre::Result<()> {
        tracing::trace!(target: "bridger", "Try execute external command");
        let config: BridgerConfig = Config::restore(Names::Bridger)?;
        let registry = config.registry;

        match registry.type_ {
            RegistryType::Local => {
                let executor = CompileSourceExecutor::new(
                    self.command.clone(),
                    self.args.clone(),
                    CompileChannel::Debug,
                    None,
                );
                executor.execute(None)
            }
            RegistryType::Github | RegistryType::Server => {
                let executor =
                    PrecompiledBinaryExecutor::new(self.command.clone(), self.args.clone());
                executor.execute(registry.path)
            }
        }
    }
}

/// The trait of subcommand executor, support mutiple provider.
pub trait ISubcommandExecutor {
    /// Execute subcommand, The parameter path is current registry path value.
    fn execute(&self, path: Option<String>) -> color_eyre::Result<()>;
}
