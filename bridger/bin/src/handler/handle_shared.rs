use std::path::Path;

use bridge_shared::shared::{BridgeShared, SharedConfig};
use bridge_standard::error::StandardError;

use crate::dc;
use crate::types::command::SharedCommand;

pub async fn handle_shared(command: SharedCommand) -> anyhow::Result<()> {
    match command {
        SharedCommand::Start { config } => {
            let saved_shared = dc::get_shared();
            if saved_shared.is_some() {
                return Ok(());
            }

            let path = Path::new(&config);
            if !path.exists() {
                return Err(StandardError::Cli(format!(
                    "The shared config not found: [{}]",
                    config
                )))?;
            }
            if !path.is_file() {
                return Err(StandardError::Cli(format!(
                    "The config path is not file: [{}]",
                    config
                )))?;
            }

            let mut c = config::Config::default();
            c.merge(config::File::from(path))?;
            let shared_config = c.try_into::<SharedConfig>().map_err(|e| {
                StandardError::Cli(format!("Failed to load shared config: {:?}", e))
            })?;
            let shared = BridgeShared::new(shared_config)?;
            dc::set_shared(shared)?;
            println!("Start shared success");
        }
    }
    Ok(())
}
