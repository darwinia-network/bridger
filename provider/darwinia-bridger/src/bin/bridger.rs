use actix::System;
use darwinia_bridger::cmd;

#[actix_rt::main]
async fn main() {
	if let Err(err) = cmd::exec().await {
		log::error!("{}", err.to_string());
		System::current().stop();
		std::process::exit(1);
	}
}
