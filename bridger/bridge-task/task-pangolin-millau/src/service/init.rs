use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use lifeline::{Bus, Lifeline, Receiver, Service, Task};
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};

use bridge_standard::bridge::service::BridgeService;
use bridge_standard::bridge::task::BridgeSand;

use crate::bus::PangolinMillauBus;
use crate::message::{BridgeName, PangolinMillauMessage};
use crate::task::PangolinMillauTask;

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
        // let config_pangolin: ChainInfoConfig =
        //     Config::restore_with_namespace(PangolinMillauTask::NAME, "pangolin")?;
        // let config_millau: ChainInfoConfig =
        //     Config::restore_with_namespace(PangolinMillauTask::NAME, "millau")?;
        let _greet = Self::try_task(
            &format!("{}-init-bridge", PangolinMillauTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        PangolinMillauMessage::InitBridge(bridge_info) => {
                            let bridge = bridge_info.bridge;
                            let source_chain = bridge_info.source_chain;
                            let target_chain = bridge_info.target_chain;
                            select_bridge!(bridge, {
                                let source_client =
                                    source_chain.to_substrate_relay_chain::<Source>().await?;
                                let target_client =
                                    target_chain.to_substrate_relay_chain::<Target>().await?;
                                let target_sign = target_chain.to_keypair::<Target>()?;
                                log::debug!("source client -> {:?}", source_client);
                                log::debug!("target client -> {:?}", target_client);

                                external_s2s::relay::headers_initialize::initialize(
                                    source_client,
                                    target_client.clone(),
                                    target_sign.public().into(),
                                    move |transaction_nonce, initialization_data| {
                                        Bytes(
                                            Target::sign_transaction(
                                                *target_client.genesis_hash(),
                                                &target_sign,
                                                transaction_nonce,
                                                encode_init_bridge(initialization_data),
                                            )
                                            .encode(),
                                        )
                                    },
                                )
                                .await;
                            });
                        }
                    }
                }
                Ok(())
            },
        );
        Ok(Self { _greet })
    }
}
