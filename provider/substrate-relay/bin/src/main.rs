#![feature(async_closure)]

#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate async_trait;
#[macro_use]
extern crate log;

use actix_web::rt::System;

mod api;
mod cli;
mod client;
mod error;
mod initialize;
mod persist;
mod s2s;
mod server;
mod types;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
	initialize::init()?;

	if let Err(err) = cli::exec().await {
		log::error!("{}", err.to_string());
		System::current().stop();
		std::process::exit(1);
	}
	Ok(())
}
