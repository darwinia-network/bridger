use futures::{FutureExt, TryFutureExt};
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use relay_utils::metrics::MetricsParams;
use substrate_relay_helper::messages_lane::{MessagesRelayParams, SubstrateMessageLane};
use substrate_relay_helper::on_demand_headers::OnDemandHeadersRelay;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::PangolinPangoroBus;
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::message::PangolinPangoroMessageSend;
use crate::task::PangolinPangoroTask;
use crate::types::{BridgeName, RelayHeadersAndMessagesInfo, WrapperRelayerMode};

#[derive(Debug)]
pub struct RelayService {
    _greet: Lifeline,
}

impl BridgeService for RelayService {}

impl Service for RelayService {
    type Bus = PangolinPangoroBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<PangolinPangoroMessageSend>()?;
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangolin")?;
        let config_pangoro: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangoro")?;
        let config_relay: RelayConfig = Config::restore(PangolinPangoroTask::NAME)?;

        let _greet = Self::try_task(
            &format!("{}-relay", PangolinPangoroTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        PangolinPangoroMessageSend::Relay(bridge) => {
                            let (source_chain, target_chain) = match bridge {
                                BridgeName::PangolinToPangoro => (
                                    config_pangolin.to_chain_info_with_expect_signer(
                                        config_relay.signer_pangolin.clone(),
                                    )?,
                                    config_pangoro.to_chain_info_with_expect_signer(
                                        config_relay.signer_pangoro.clone(),
                                    )?,
                                ),
                                BridgeName::PangoroToPangolin => (
                                    config_pangoro.to_chain_info_with_expect_signer(
                                        config_relay.signer_pangoro.clone(),
                                    )?,
                                    config_pangolin.to_chain_info_with_expect_signer(
                                        config_relay.signer_pangolin.clone(),
                                    )?,
                                ),
                            };
                            let relay_info = RelayHeadersAndMessagesInfo {
                                bridge,
                                source: source_chain,
                                target: target_chain,
                                lanes: config_relay.lanes.clone(),
                                prometheus_params: config_relay.prometheus_params.clone(),
                                relayer_mode: config_relay
                                    .relayer_mode
                                    .unwrap_or(WrapperRelayerMode::Rational)
                                    .into(),
                            };

                            std::thread::spawn(move || {
                                futures::executor::block_on(bridge_relay(relay_info))
                            })
                            .join()
                            .map_err(|_| anyhow::Error::msg("Failed to join thread handle"))??;
                        }
                        _ => continue,
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            BridgeName::PangolinToPangoro => {
                type Left = component_pangolin_s2s::PangolinChain;
                type Right = component_pangoro_s2s::PangoroChain;

                type LeftToRightFinality = crate::chains::pangolin::PangolinFinalityToPangoro;
                type RightToLeftFinality = crate::chains::pangoro::PangoroFinalityToPangolin;

                type LeftToRightMessages = crate::chains::pangolin::PangolinMessagesToPangoro;
                type RightToLeftMessages = crate::chains::pangoro::PangoroMessagesToPangolin;

                const MAX_MISSING_LEFT_HEADERS_AT_RIGHT: common_primitives::BlockNumber =
                    pangolin_constants::BLOCKS_PER_SESSION;
                const MAX_MISSING_RIGHT_HEADERS_AT_LEFT: common_primitives::BlockNumber =
                    pangoro_constants::BLOCKS_PER_SESSION;

                use crate::chains::pangolin::PangolinMessagesToPangoroRunner as left_to_right_messages_runner;
                use crate::chains::pangoro::PangoroMessagesToPangolinRunner as right_to_left_messages_runner;

                $generic
            }
            _ => anyhow::bail!("Not support bridge {:?}", $bridge),
        }
    };
}

async fn bridge_relay(relay_info: RelayHeadersAndMessagesInfo) -> anyhow::Result<()> {
    let bridge = relay_info.bridge;
    let source_chain = relay_info.source;
    let target_chain = relay_info.target;
    let relayer_mode = relay_info.relayer_mode;
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
                relayer_mode,
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<
                        <LeftToRightMessages as SubstrateMessageLane>::MessageLane,
                    >(&lane),
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
                relayer_mode,
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<
                        <RightToLeftMessages as SubstrateMessageLane>::MessageLane,
                    >(&lane),
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
