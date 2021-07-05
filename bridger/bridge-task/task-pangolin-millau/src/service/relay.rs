use std::convert::TryFrom;

use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_standard::bridge::config::Config;
use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;
use external_s2s::runner::pangolin_millau;
use external_s2s::types::{ChainInfo, RelayHeadersAndMessagesInfo};

use crate::bus::PangolinMillauBus;
use crate::config::{ChainInfoConfig, RelayConfig};
use crate::message::{BridgeName, PangolinMillauMessage};
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
        let mut rx = bus.rx::<PangolinMillauMessage>()?;
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "pangolin")?;
        let config_millau: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "millau")?;
        let config_relay: RelayConfig = Config::restore(PangolinMillauTask::NAME)?;

        let _greet = Self::try_task(&format!("{}-relay", PangolinMillauTask::NAME), async move {
            while let Some(message) = rx.recv().await {
                match message {
                    PangolinMillauMessage::Relay(bridge) => {
                        let source_chain = if bridge == BridgeName::PangolinToMillau {
                            config_pangolin.clone()
                        } else {
                            config_millau.clone()
                        };
                        let target_chain = if bridge == BridgeName::MillauToPangolin {
                            config_millau.clone()
                        } else {
                            config_pangolin.clone()
                        };
                        let relay_info = RelayHeadersAndMessagesInfo {
                            bridge,
                            source: ChainInfo::try_from(source_chain)?,
                            target: ChainInfo::try_from(target_chain)?,
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
