use crate::error;

pub fn init() -> error::Result<()> {
    init_log();
    init_relay_chain();
    Ok(())
}

fn init_log() {
    std::env::set_var(
        "RUST_LOG",
        "serde=info,actix_web=info,substrate_relay=debug",
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

// init relay chain and registry to persist
fn init_relay_chain() -> error::Result<()> {
    Ok(())
}
