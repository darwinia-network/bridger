use support_config::{Config, Names};

use crate::command::output;
use crate::command::types::RegistryType;
use crate::config::BridgerConfig;

pub fn handle_registry(type_: RegistryType, path: Option<String>) -> color_eyre::Result<()> {
    let mut config: BridgerConfig = Config::restore(Names::Bridger)?;
    config.registry.type_ = type_;
    config.registry.path = path;
    Config::store(Names::Bridger, config)?;
    output::output_ok();
    Ok(())
}
