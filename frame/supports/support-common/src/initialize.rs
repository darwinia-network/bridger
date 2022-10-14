use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::EnvFilter;

pub fn init() -> color_eyre::Result<()> {
    init_log()?;
    Ok(())
}

fn init_log() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // if std::env::var("RUST_BACKTRACE").is_err() {
    //     std::env::set_var("RUST_SPANTRACE", "1");
    //     std::env::set_var("RUST_LIB_BACKTRACE", "full");
    // }

    let def_log_filter = [
        "info",
        "jsonrpsee_ws_client=error",
        "hyper=error",
        "isahc=error",
        "lifeline=debug",
        "bridger=error",
        "pangolin-pangoro=trace",
        "pangolin-ropsten=trace",
        "pangoro-chapel=trace",
        "crab-crabparachain=trace",
        "pangolin-pangolinparachain=trace",
        "pangolin-pangolinparachainalpha=trace",
        "darwinia-ethereum=trace",
        "darwinia-crab=trace",
        "client-pangolin=trace",
        "client-pangoro=trace",
        "feemarket=trace",
        "shadow=trace",
        "relay-s2s=info",
    ]
    .join(",");

    let use_json_adapter = std::env::var("LOG_ADAPTER")
        .map(|v| &v.to_lowercase()[..] == "json")
        .unwrap_or_default();
    let max_log_level = std::env::var("LOG_MAX_LEVEL")
        .map(|v| Level::from_str(&v).unwrap_or(Level::TRACE))
        .unwrap_or(Level::TRACE);

    if use_json_adapter {
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(max_log_level)
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::from(def_log_filter)),
            )
            .json()
            .init();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");
        return Ok(());
    }

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(max_log_level)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from(def_log_filter)),
        )
        // https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
        .with_timer(ChronoUtc::with_format("%F %T".to_string()))
        .init();
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    Ok(())
}
