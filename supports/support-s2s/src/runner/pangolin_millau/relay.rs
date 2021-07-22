use futures::{FutureExt, TryFutureExt};
use relay_utils::metrics::MetricsParams;

use crate::relay::messages_lane::MessagesRelayParams;
use crate::relay::on_demand_headers::OnDemandHeadersRelay;
use crate::types::BridgeName;
use crate::types::RelayHeadersAndMessagesInfo;

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            BridgeName::PangolinToMillau => {
                type Left = component_pangolin::PangolinChain;
                type Right = component_millau::MillauChain;

                type LeftToRightFinality = crate::declaration::pangolin::PangolinFinalityToMillau;
                type RightToLeftFinality = crate::declaration::millau::MillauFinalityToPangolin;

                type LeftToRightMessages = crate::declaration::pangolin::PangolinMessagesToMillau;
                type RightToLeftMessages = crate::declaration::millau::MillauMessagesToPangolin;

                const MAX_MISSING_LEFT_HEADERS_AT_RIGHT: drml_primitives::BlockNumber =
                    pangolin_constants::BLOCKS_PER_SESSION;
                const MAX_MISSING_RIGHT_HEADERS_AT_LEFT: millau_primitives::BlockNumber =
                    millau_primitives::SESSION_LENGTH;

                use crate::declaration::pangolin::PangolinMessagesToMillauRunner as left_to_right_messages_runner;
                use crate::declaration::millau::MillauMessagesToPangolinRunner as right_to_left_messages_runner;

                $generic
            }
            _ => anyhow::bail!("Not support bridge {:?}", $bridge),
        }
    };
}

pub async fn bridge_relay(relay_info: RelayHeadersAndMessagesInfo) -> anyhow::Result<()> {
    let bridge = relay_info.bridge;
    let source_chain = relay_info.source;
    let target_chain = relay_info.target;
    select_bridge!(bridge, {
        let left_client = source_chain.to_substrate_relay_chain::<Left>().await?;
        let right_client = target_chain.to_substrate_relay_chain::<Right>().await?;

        let left_sign = source_chain.to_keypair::<Left>()?;
        let right_sign = target_chain.to_keypair::<Right>()?;

        let lanes = relay_info.lanes;

        let metrics_params: MetricsParams = relay_info.prometheus_params.clone().into();
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
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<LeftToRightMessages>(&lane),
                ),
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
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<RightToLeftMessages>(&lane),
                ),
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
