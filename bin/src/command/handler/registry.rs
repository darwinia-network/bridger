use support_config::{Config, Names};

use crate::command::types::RegistryType;
use crate::config::BridgerConfig;

pub fn handle_registry(type_: RegistryType, path: Option<String>) -> color_eyre::Result<()> {
    let config: BridgerConfig = Config::restore(Names::Bridger)?;
    println!("===> {:?}", config);
    Ok(())
}
