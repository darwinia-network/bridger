use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use support_s2s::runner::pangolin_millau;
use support_s2s::types::RelayHeadersAndMessagesInfo;

use crate::bus::PangolinMillauBus;
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::message::{BridgeName, PangolinMillauMessageSend};
use crate::task::PangolinMillauTask;

#[derive(Debug)]
pub struct RelayService {
    _greet: Lifeline,
}

impl BridgeService for RelayService {}

impl Service for RelayService {
    type Bus = PangolinMillauBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<PangolinMillauMessageSend>()?;
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "pangolin")?;
        let config_millau: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "millau")?;
        let config_relay: RelayConfig = Config::restore(PangolinMillauTask::NAME)?;

        let _greet = Self::try_task(&format!("{}-relay", PangolinMillauTask::NAME), async move {
            while let Some(message) = rx.recv().await {
                match message {
                    PangolinMillauMessageSend::Relay(bridge) => {
                        let (source_chain, target_chain) = match bridge {
                            BridgeName::PangolinToMillau => (
                                config_pangolin.to_chain_info_with_expect_signer(
                                    config_relay.signer_pangolin.clone(),
                                )?,
                                config_millau.to_chain_info_with_expect_signer(
                                    config_relay.signer_millau.clone(),
                                )?,
                            ),
                            BridgeName::MillauToPangolin => (
                                config_millau.to_chain_info_with_expect_signer(
                                    config_relay.signer_millau.clone(),
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
                        };
                        pangolin_millau::bridge_relay(relay_info).await?;
                    }
                    _ => continue,
                }
            }
            Ok(())
        });
        Ok(Self { _greet })
    }
}
