#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

use actix_web::rt::System;

mod cli;
mod error;

fn init() {
	std::env::set_var(
		"RUST_LOG",
		"serde=info,actix_web=info,substrate_relay=debug",
	);
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	self::init();

	if let Err(err) = cli::exec().await {
		log::error!("{}", err.to_string());
		System::current().stop();
		std::process::exit(1);
	}
	Ok(())
}
