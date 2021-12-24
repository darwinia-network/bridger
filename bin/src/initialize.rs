use support_config::{Config, ConfigFormat, Names};

use crate::config::BridgerConfig;

pub fn init() -> color_eyre::Result<()> {
    support_initialize::init()?;
    init_default_config()?;
    Ok(())
}

fn init_default_config() -> color_eyre::Result<()> {
    if Config::exists(Names::Bridger)? {
        return Ok(());
    }
    let config = BridgerConfig::default();
    Config::store_with_format(Names::Bridger, config, ConfigFormat::Toml)
}
