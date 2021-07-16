use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;
use support_s2s::runner::pangolin_millau;
use support_s2s::types::InitBridge;

use crate::bus::PangolinMillauBus;
use crate::config::ChainInfoConfig;
use crate::message::{BridgeName, PangolinMillauMessageReceive, PangolinMillauMessageSend};
use crate::task::PangolinMillauTask;

#[derive(Debug)]
pub struct InitBridgeService {
    _greet: Lifeline,
}

impl BridgeService for InitBridgeService {}

impl Service for InitBridgeService {
    type Bus = PangolinMillauBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<PangolinMillauMessageSend>()?;
        let mut tx = bus.tx::<PangolinMillauMessageReceive>()?;
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "pangolin")?;
        let config_millau: ChainInfoConfig =
            Config::restore_with_namespace(PangolinMillauTask::NAME, "millau")?;

        let _greet = Self::try_task(
            &format!("{}-init-bridge", PangolinMillauTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        PangolinMillauMessageSend::InitBridge(bridge) => {
                            let (source_chain, target_chain) = match bridge {
                                BridgeName::PangolinToMillau => (
                                    config_pangolin.to_chain_info()?,
                                    config_millau.to_chain_info()?,
                                ),
                                BridgeName::MillauToPangolin => (
                                    config_millau.to_chain_info()?,
                                    config_pangolin.to_chain_info()?,
                                ),
                            };
                            let init_bridge = InitBridge {
                                bridge,
                                source: source_chain,
                                target: target_chain,
                            };
                            pangolin_millau::init_bridge(init_bridge).await?;
                            tx.send(PangolinMillauMessageReceive::FinishedInitBridge)
                                .await?;
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
