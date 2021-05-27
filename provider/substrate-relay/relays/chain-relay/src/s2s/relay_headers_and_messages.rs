use futures::{FutureExt, TryFutureExt};
use relay_utils::metrics::MetricsParams;

use crate::types::transfer::{BridgeName, ChainInfo};
use crate::{
	types::s2s::{messages_lane::MessagesRelayParams, on_demand_headers::OnDemandHeadersRelay},
	types::transfer::RelayHeadersAndMessagesInfo,
};

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			BridgeName::PangolinToMillau => {
				type Left = pangolin_bridge_relay_client_definition::PangolinChain;
				type Right = relay_millau_client::Millau;

				type LeftToRightFinality = crate::PangolinFinalityToMillau;
				type RightToLeftFinality = crate::MillauFinalityToPangolin;

				type LeftToRightMessages = crate::PangolinMessagesToMillau;
				type RightToLeftMessages = crate::MillauMessagesToPangolin;

				// todo: The BlockNumber can be define in ChainConst
				const MAX_MISSING_LEFT_HEADERS_AT_RIGHT: drml_primitives::BlockNumber =
					pangolin_constants::BLOCKS_PER_SESSION;
				const MAX_MISSING_RIGHT_HEADERS_AT_LEFT: bp_millau::BlockNumber = bp_millau::SESSION_LENGTH;

				use crate::MillauMessagesToPangolinRunner as right_to_left_messages_runner;
				use crate::PangolinMessagesToMillauRunner as left_to_right_messages_runner;

				$generic
			}
			_ => {
				anyhow::bail!("Not support bridge {:?}", $bridge);
			}
		}
	};
}

pub async fn run(relay_info: RelayHeadersAndMessagesInfo) -> anyhow::Result<()> {
	let source_chain: &ChainInfo = relay_info.source();
	let target_chain: &ChainInfo = relay_info.target();
	let bridge: &BridgeName = relay_info.bridge();

	info!("Relay headers and messages {:?}", bridge);

	select_bridge!(bridge, {
		let left_client = source_chain.to_substrate_relay_chain::<Left>().await?;
		let right_client = target_chain.to_substrate_relay_chain::<Right>().await?;

		let left_sign = source_chain.to_keypair::<Left>()?;
		let right_sign = target_chain.to_keypair::<Right>()?;

		let lanes = relay_info.lanes().clone();

		let metrics_params: MetricsParams = relay_info.prometheus_params().clone().into();
		let metrics_params = relay_utils::relay_metrics(None, metrics_params).into_params();

		let left_to_right_on_demand_headers = OnDemandHeadersRelay::new(
			left_client.clone(),
			right_client.clone(),
			LeftToRightFinality::new(right_client.clone(), right_sign.clone()),
			MAX_MISSING_LEFT_HEADERS_AT_RIGHT,
		);
		let right_to_left_on_demand_headers = OnDemandHeadersRelay::new(
			right_client.clone(),
			left_client.clone(),
			RightToLeftFinality::new(left_client.clone(), left_sign.clone()),
			MAX_MISSING_RIGHT_HEADERS_AT_LEFT,
		);

		// Need 2x capacity since we consider both directions for each lane
		let mut message_relays = Vec::with_capacity(lanes.len() * 2);
		for lane in lanes {
			let lane = lane.into();

			let left_to_right_messages = left_to_right_messages_runner::run(MessagesRelayParams {
				source_client: left_client.clone(),
				source_sign: left_sign.clone(),
				target_client: right_client.clone(),
				target_sign: right_sign.clone(),
				source_to_target_headers_relay: Some(left_to_right_on_demand_headers.clone()),
				target_to_source_headers_relay: Some(right_to_left_on_demand_headers.clone()),
				lane_id: lane,
				metrics_params: metrics_params
					.clone()
					.disable()
					.metrics_prefix(messages_relay::message_lane_loop::metrics_prefix::<LeftToRightMessages>(&lane)),
			})
			.map_err(|e| anyhow::format_err!("{}", e))
			.boxed();

			let right_to_left_messages = right_to_left_messages_runner::run(MessagesRelayParams {
				source_client: right_client.clone(),
				source_sign: right_sign.clone(),
				target_client: left_client.clone(),
				target_sign: left_sign.clone(),
				source_to_target_headers_relay: Some(right_to_left_on_demand_headers.clone()),
				target_to_source_headers_relay: Some(left_to_right_on_demand_headers.clone()),
				lane_id: lane,
				metrics_params: metrics_params
					.clone()
					.disable()
					.metrics_prefix(messages_relay::message_lane_loop::metrics_prefix::<RightToLeftMessages>(&lane)),
			})
			.map_err(|e| anyhow::format_err!("{}", e))
			.boxed();

			message_relays.push(left_to_right_messages);
			message_relays.push(right_to_left_messages);
		}

		relay_utils::relay_metrics(None, metrics_params)
			.expose()
			.await
			.map_err(|e| anyhow::format_err!("{}", e))?;

		futures::future::select_all(message_relays).await.0
	})
}
