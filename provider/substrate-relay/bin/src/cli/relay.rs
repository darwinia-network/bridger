use crate::cli::types::OptRelay;
use crate::client::cli_client::CliClient;
use crate::error;
use crate::types::cond::relay::StartRelayCond;

pub async fn exec(opt_relay: OptRelay) -> error::Result<()> {
	match opt_relay {
		OptRelay::Start {
			bridge_info,
			lane,
			prometheus_params,
		} => {
			let server = bridge_info.server;
			let token = bridge_info.token;
			let source = bridge_info.source;
			let target = bridge_info.target;

			let client = CliClient::new(server.clone(), token.clone(), false);
			let data = StartRelayCond::builder()
				.source(source)
				.target(target)
				.lance(lane)
				.no_prometheus(prometheus_params.no_prometheus)
				.prometheus_host(prometheus_params.prometheus_host)
				.prometheus_port(prometheus_params.prometheus_port)
				.build();
			return client.start(&data).await;
		}
		OptRelay::Status { bridge_info } => {}
	}
	Ok(())
}
