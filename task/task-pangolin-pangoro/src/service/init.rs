use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use relay_substrate_client::{
    Chain as RelaySubstrateClientChain, TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{Bytes, Pair};

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

                            std::thread::spawn(move || {
                                futures::executor::block_on(init_bridge(InitBridge {
                                    bridge,
                                    source: source_chain,
                                    target: target_chain,
                                }))
                            })
                            .join()
                            .map_err(|_| anyhow::Error::msg("Failed to join thread handle"))??;

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

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            BridgeName::PangolinToPangoro => {
                type Source = component_pangolin_s2s::PangolinChain;
                type Target = component_pangoro_s2s::PangoroChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    pangoro_runtime::BridgeGrandpaCall::<
                        pangoro_runtime::Runtime,
                        pangoro_runtime::WithPangolinGrandpa,
                    >::initialize(init_data)
                    .into()
                }

                $generic
            }
            BridgeName::PangoroToPangolin => {
                type Source = component_pangoro_s2s::PangoroChain;
                type Target = component_pangolin_s2s::PangolinChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    pangolin_runtime::BridgeGrandpaCall::<
                        pangolin_runtime::Runtime,
                        pangolin_runtime::WithPangoroGrandpa,
                    >::initialize(init_data)
                    .into()
                }

                $generic
            }
        }
    };
}

async fn init_bridge(init_bridge: InitBridge) -> anyhow::Result<()> {
    let bridge = init_bridge.bridge;
    let source_chain = init_bridge.source;
    let target_chain = init_bridge.target;
    select_bridge!(bridge, {
        let source_client = source_chain.to_substrate_relay_chain::<Source>().await?;
        let target_client = target_chain.to_substrate_relay_chain::<Target>().await?;
        let target_sign = target_chain.to_keypair::<Target>()?;
        log::debug!(
            target: PangolinPangoroTask::NAME,
            "source client -> {:?}",
            source_client
        );
        log::debug!(
            target: PangolinPangoroTask::NAME,
            "target client -> {:?}",
            target_client
        );

        substrate_relay_helper::headers_initialize::initialize(
            source_client,
            target_client.clone(),
            target_sign.public().into(),
            move |transaction_nonce, initialization_data| {
                Bytes(
                    Target::sign_transaction(
                        *target_client.genesis_hash(),
                        &target_sign,
                        relay_substrate_client::TransactionEra::immortal(),
                        UnsignedTransaction::new(
                            encode_init_bridge(initialization_data),
                            transaction_nonce,
                        ),
                    )
                    .encode(),
                )
            },
        )
        .await;
    });
    Ok(())
}
