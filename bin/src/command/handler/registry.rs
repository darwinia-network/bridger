use support_config::{Config, Names};

use crate::command::output;
use crate::command::output::output_err_and_exit;
use crate::command::types::RegistryType;
use crate::config::BridgerConfig;

pub fn handle_registry(type_: RegistryType, mut path: Option<String>) -> color_eyre::Result<()> {
    if type_ == RegistryType::Github && path.is_none() {
        path = Some("https://github.com/darwinia-network/bridger".to_string());
    }
    if type_ != RegistryType::Local && path.is_none() {
        output_err_and_exit("Please provide `--path <path>`");
    }
    let mut config: BridgerConfig = Config::restore(Names::Bridger)?;
    config.registry.type_ = type_;
    config.registry.path = path;
    Config::store(Names::Bridger, config)?;
    output::output_ok();
    Ok(())
}
