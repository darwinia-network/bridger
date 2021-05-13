use relay_utils::metrics::MetricsParams;

use crate::{types::transfer::ChainInfo, RelayChain, RelayChainMillau, RelayChainPangolin};

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			("pangolin", "millau") => {
				type Left = <RelayChainPangolin as RelayChain>::Chain;
				type Right = <RelayChainMillau as RelayChain>::Chain;

				type LeftToRightFinality = crate::PangolinFinalityToMillau;
				type RightToLeftFinality = crate::MillauFinalityToPangolin;

				type LeftToRightMessages = crate::PangolinMessagesToMillau;
				type RightToLeftMessages = crate::MillauMessagesToPangolin;

				// todo: The BlockNumber can be define in ChainConst
				const MAX_MISSING_LEFT_HEADERS_AT_RIGHT: drml_primitives::BlockNumber =
					pangolin_runtime_params::s2s::SESSION_LENGTH;
				const MAX_MISSING_RIGHT_HEADERS_AT_LEFT: bp_millau::BlockNumber = bp_millau::SESSION_LENGTH;

				let right_to_left_messages = crate::MillauMessagesToPangolinRunner::run;
				let left_to_right_messages = crate::PangolinMessagesToMillauRunner::run;

				$generic
			}
			_ => {
				anyhow::bail!(
					"Not support bridge {} -> {}",
					$bridge.0.to_string(),
					$bridge.1.to_string(),
				);
			}
		}
	};
}

pub async fn run(source_chain: ChainInfo, target_chain: ChainInfo) -> anyhow::Result<()> {
	let bridge = (&source_chain.name()[..], &target_chain.name()[..]);
	info!("Relay headers and messages {} -> {}", bridge.0, bridge.1);
	select_bridge!(bridge, {
		let left_client = source_chain.to_substrate_relay_chain::<Left>().await?;
		let right_client = target_chain.to_substrate_relay_chain::<Right>().await?;

		let left_sign = source_chain.to_keypair::<Left>()?;
		let right_sign = target_chain.to_keypair::<Right>()?;

		// todo: need lance support
		// let lane = params.shared.lane.into();

		// let metrics_params: MetricsParams = params.shared.prometheus_params.into();
		// let metrics_params = relay_utils::relay_metrics(None, metrics_params).into_params();
	});
	Ok(())
}
