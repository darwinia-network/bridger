use colored::Colorize;

use crate::command::types::RegistryType;
use crate::config::BridgerConfig;
use support_common::error::BridgerError;
use support_config::{Config, Names};

use crate::external;
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
        let config: BridgerConfig = Config::restore(Names::Bridger)?;
        let registry = config.registry;

        match registry.type_ {
            RegistryType::Local => {
                let executor = CompileSourceExecutor::new(
                    self.command.clone(),
                    self.args.clone(),
                    CompileChannel::Debug,
                );
                executor.execute(None)
            }
            RegistryType::Github | RegistryType::Server => {
                let executor = PrecompiledBinaryExecutor::new();
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
