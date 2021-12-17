use crate::config::BridgerConfig;
use support_config::{Config, ConfigFormat, Names};
use tracing::Level;

pub fn init() -> color_eyre::Result<()> {
    init_log()?;
    init_default_config()?;
    Ok(())
}

fn init_log() -> color_eyre::Result<()> {
    color_eyre::install()?;
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "1");
        std::env::set_var("RUST_LIB_BACKTRACE", "full");
    }
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        let filter = "trace,hyper=error";
        std::env::set_var("RUST_LOG", filter);
        filter.to_string()
    });

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .with_env_filter(log_filter)
        // builds the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    Ok(())
}

fn init_default_config() -> color_eyre::Result<()> {
    if Config::exists(Names::Bridger)? {
        return Ok(());
    }
    let config = BridgerConfig::default();
    Config::store_with_format(Names::Bridger, config, ConfigFormat::Toml)
}
