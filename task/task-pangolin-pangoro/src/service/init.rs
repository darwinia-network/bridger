use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::PangolinPangoroBus;
use crate::config::ChainInfoConfig;
use crate::message::{PangolinPangoroMessageReceive, PangolinPangoroMessageSend};
use crate::task::PangolinPangoroTask;
use crate::types::{BridgeName, InitBridge};

#[derive(Debug)]
pub struct InitBridgeService {
    _greet: Lifeline,
}

impl BridgeService for InitBridgeService {}

impl Service for InitBridgeService {
    type Bus = PangolinPangoroBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<PangolinPangoroMessageSend>()?;
        let mut tx = bus.tx::<PangolinPangoroMessageReceive>()?;
        let config_pangolin: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangolin")?;
        let config_pangoro: ChainInfoConfig =
            Config::restore_with_namespace(PangolinPangoroTask::NAME, "pangoro")?;

        let _greet = Self::try_task(
            &format!("{}-init-bridge", PangolinPangoroTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        PangolinPangoroMessageSend::InitBridge(bridge) => {
                            let (source_chain, target_chain) = match bridge {
                                BridgeName::PangolinToPangoro => (
                                    config_pangolin.to_chain_info()?,
                                    config_pangoro.to_chain_info()?,
                                ),
                                BridgeName::PangoroToPangolin => (
                                    config_pangoro.to_chain_info()?,
                                    config_pangolin.to_chain_info()?,
                                ),
                            };
                            let init_bridge = InitBridge {
                                bridge,
                                source: source_chain,
                                target: target_chain,
                            };
                            // todo: init bridge
                            // pangolin_pangoro::init_bridge(init_bridge).await?;
                            tx.send(PangolinPangoroMessageReceive::FinishedInitBridge)
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
