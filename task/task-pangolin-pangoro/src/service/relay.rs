use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::PangolinPangoroBus;
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::message::PangolinPangoroMessageSend;
use crate::task::PangolinPangoroTask;
use crate::types::{BridgeName, RelayHeadersAndMessagesInfo};

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
                            };
                            // todo: start relay
                            // pangolin_pangoro::bridge_relay(relay_info).await?;
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
