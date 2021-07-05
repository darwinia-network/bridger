use lifeline::{Bus, Lifeline, Receiver, Service, Task};

use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;

use crate::bus::PangolinMillauBus;
use crate::message::PangolinMillauMessage;
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
        let mut rx = bus.rx::<PangolinMillauMessage>()?;
        let _greet = Self::try_task(
            &format!("{}-init-bridge", PangolinMillauTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        PangolinMillauMessage::InitBridge(name) => {
                            //
                        }
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
            BridgeName::PangolinToMillau => {
                type Source = chain_pangolin::PangolinChain;
                type Target = chain_millau::MillauChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    let initialize_call = millau_runtime::BridgeGrandpaCall::<
                        millau_runtime::Runtime,
                        millau_runtime::WithPangolinGrandpa,
                    >::initialize(init_data);
                    millau_runtime::SudoCall::sudo(Box::new(initialize_call.into())).into()
                }

                $generic
            }
            BridgeName::MillauToPangolin => {
                type Source = chain_millau::MillauChain;
                type Target = chain_pangolin::PangolinChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    let initialize_call = pangolin_runtime::BridgeGrandpaCall::<
                        pangolin_runtime::Runtime,
                        pangolin_runtime::WithMillauGrandpa,
                    >::initialize(init_data);
                    pangolin_runtime::SudoCall::sudo(Box::new(initialize_call.into())).into()
                }

                $generic
            }
        }
    };
}
