use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::EnvFilter;

pub fn init() -> color_eyre::Result<()> {
    init_log()?;
    Ok(())
}

fn init_log() -> color_eyre::Result<()> {
    color_eyre::install()?;
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "1");
        std::env::set_var("RUST_LIB_BACKTRACE", "full");
    }

    let def_log_filter = ["trace", "hyper=error"].join(",");

    let use_json_adapter = std::env::var("LOG_ADAPTER")
        .map(|v| &v.to_lowercase()[..] == "json")
        .unwrap_or_default();
    let max_log_level = std::env::var("LOG_MAX_LEVEL")
        .map(|v| Level::from_str(&v).unwrap_or(Level::TRACE))
        .unwrap_or(Level::TRACE);

    if use_json_adapter {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(max_log_level)
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::from(def_log_filter)),
            )
            .json()
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        return Ok(());
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(max_log_level)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from(def_log_filter)),
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    Ok(())
}
