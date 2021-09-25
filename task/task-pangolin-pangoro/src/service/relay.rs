use futures::{FutureExt, TryFutureExt};
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use relay_utils::metrics::MetricsParams;
use substrate_relay_helper::messages_lane::{MessagesRelayParams, SubstrateMessageLane};
use substrate_relay_helper::on_demand_headers::OnDemandHeadersRelay;

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use component_pangolin_s2s::PangolinChain;
use component_pangoro_s2s::PangoroChain;

use crate::bus::PangolinPangoroBus;
use crate::chains::pangolin::{
    PangolinFinalityToPangoro, PangolinMessagesToPangoro, PangolinMessagesToPangoroRunner,
};
use crate::chains::pangoro::{
    PangoroFinalityToPangolin, PangoroMessagesToPangolin, PangoroMessagesToPangolinRunner,
};
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::message::PangolinPangoroMessageSend;
use crate::task::PangolinPangoroTask;
use crate::types::{RelayHeadersAndMessagesInfo, WrapperRelayerMode};

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
                while let Some(_message) = rx.recv().await {
                    let (source_chain, target_chain) = (
                        config_pangolin.to_chain_info_with_expect_signer(
                            config_relay.signer_pangolin.clone(),
                        )?,
                        config_pangoro.to_chain_info_with_expect_signer(
                            config_relay.signer_pangoro.clone(),
                        )?,
                    );

                    let relay_info = RelayHeadersAndMessagesInfo {
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
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}

async fn bridge_relay(relay_info: RelayHeadersAndMessagesInfo) -> anyhow::Result<()> {
    let pangolin_chain = relay_info.source;
    let pangoro_chain = relay_info.target;
    let relayer_mode = relay_info.relayer_mode;

    let pangolin_client = pangolin_chain
        .to_substrate_relay_chain::<PangolinChain>()
        .await?;
    let pangoro_client = pangoro_chain
        .to_substrate_relay_chain::<PangoroChain>()
        .await?;

    let pangolin_sign = pangolin_chain.to_keypair::<PangolinChain>()?;
    let pangoro_sign = pangoro_chain.to_keypair::<PangoroChain>()?;
    let pangolin_transactions_mortality = pangolin_chain.transactions_mortality()?;
    let pangoro_transactions_mortality = pangoro_chain.transactions_mortality()?;

    let lanes = relay_info.lanes;

    let metrics_params: MetricsParams = relay_info.prometheus_params.clone().into();
    let metrics_params = relay_utils::relay_metrics(None, metrics_params).into_params();

    let pangolin_to_pangoro_on_demand_headers = OnDemandHeadersRelay::new(
        pangolin_client.clone(),
        pangoro_client.clone(),
        pangoro_transactions_mortality,
        PangolinFinalityToPangoro::new(pangoro_client.clone(), pangoro_sign.clone()),
        pangolin_constants::BLOCKS_PER_SESSION,
    );
    let pangoro_to_pangolin_on_demand_headers = OnDemandHeadersRelay::new(
        pangoro_client.clone(),
        pangolin_client.clone(),
        pangolin_transactions_mortality,
        PangoroFinalityToPangolin::new(pangolin_client.clone(), pangolin_sign.clone()),
        pangoro_constants::BLOCKS_PER_SESSION,
    );

    // Need 2x capacity since we consider both directions for each lane
    let mut message_relays = Vec::with_capacity(lanes.len() * 2);
    for lane in lanes {
        let lane = lane.into();

        let pangolin_to_pangoro_messages =
            PangolinMessagesToPangoroRunner::run(MessagesRelayParams {
                source_client: pangolin_client.clone(),
                source_sign: pangolin_sign.clone(),
                target_client: pangoro_client.clone(),
                target_sign: pangoro_sign.clone(),
                source_to_target_headers_relay: Some(pangolin_to_pangoro_on_demand_headers.clone()),
                target_to_source_headers_relay: Some(pangoro_to_pangolin_on_demand_headers.clone()),
                lane_id: lane,
                relayer_mode,
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<
                        <PangolinMessagesToPangoro as SubstrateMessageLane>::MessageLane,
                    >(&lane),
                ),
            })
            .map_err(|e| anyhow::format_err!("{}", e))
            .boxed();

        let pangoro_to_pangolin_messages =
            PangoroMessagesToPangolinRunner::run(MessagesRelayParams {
                source_client: pangoro_client.clone(),
                source_sign: pangoro_sign.clone(),
                target_client: pangolin_client.clone(),
                target_sign: pangolin_sign.clone(),
                source_to_target_headers_relay: Some(pangoro_to_pangolin_on_demand_headers.clone()),
                target_to_source_headers_relay: Some(pangolin_to_pangoro_on_demand_headers.clone()),
                lane_id: lane,
                relayer_mode,
                metrics_params: metrics_params.clone().disable().metrics_prefix(
                    messages_relay::message_lane_loop::metrics_prefix::<
                        <PangoroMessagesToPangolin as SubstrateMessageLane>::MessageLane,
                    >(&lane),
                ),
            })
            .map_err(|e| anyhow::format_err!("{}", e))
            .boxed();

        message_relays.push(pangolin_to_pangoro_messages);
        message_relays.push(pangoro_to_pangolin_messages);
    }

    relay_utils::relay_metrics(None, metrics_params)
        .expose()
        .await
        .map_err(|e| anyhow::format_err!("{}", e))?;

    futures::future::select_all(message_relays).await.0
}
