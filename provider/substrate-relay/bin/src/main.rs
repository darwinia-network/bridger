#![feature(async_closure)]

extern crate log;

use structopt::StructOpt;

use crate::types::Opt;

mod handler;
mod types;

fn init() {
	std::env::set_var(
		"RUST_LOG",
		r#"
		serde=info,
		actix_web=info,
		substrate_relay=debug,
		chain_relay=debug,
		bridge=info,
		"#,
	);
	std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();
}

async fn exec() -> anyhow::Result<()> {
	let opt = Opt::from_args();
	return match opt {
		Opt::InitBridge { bridge } => handler::init_bridge(bridge).await,
		Opt::Relay {
			bridge,
			lanes,
			prometheus,
		} => handler::on_demand_relay(bridge, lanes, prometheus).await,
	};
}

fn main() -> anyhow::Result<()> {
	init();
	futures::executor::block_on(exec())
}
