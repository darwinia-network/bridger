use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use lifeline::{Bus, Lifeline, Receiver, Sender, Service, Task};
use relay_substrate_client::{
    Chain as RelaySubstrateClientChain, SignParam, TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{Bytes, Pair};

use bridge_traits::bridge::config::Config;
use bridge_traits::bridge::service::BridgeService;
use bridge_traits::bridge::task::BridgeSand;

use crate::bus::DarwiniaCrabBus;
use crate::config::ChainInfoConfig;
use crate::message::{DarwiniaCrabMessageReceive, DarwiniaCrabMessageSend};
use crate::task::DarwiniaCrabTask;
use crate::types::{BridgeName, InitBridge};

#[derive(Debug)]
pub struct InitBridgeService {
    _greet: Lifeline,
}

impl BridgeService for InitBridgeService {}

impl Service for InitBridgeService {
    type Bus = DarwiniaCrabBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<DarwiniaCrabMessageSend>()?;
        let mut tx = bus.tx::<DarwiniaCrabMessageReceive>()?;
        let config_darwinia: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(DarwiniaCrabTask::NAME, "darwinia")?;
        let config_crab: ChainInfoConfig =
            Config::restore_with_namespace_unwrap(DarwiniaCrabTask::NAME, "crab")?;

        let _greet = Self::try_task(
            &format!("{}-init-bridge", DarwiniaCrabTask::NAME),
            async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        DarwiniaCrabMessageSend::InitBridge(bridge) => {
                            let (source_chain, target_chain) = match bridge {
                                BridgeName::DarwiniaToCrab => (
                                    config_darwinia.to_chain_info()?,
                                    config_crab.to_chain_info()?,
                                ),
                                BridgeName::CrabToDarwinia => (
                                    config_crab.to_chain_info()?,
                                    config_darwinia.to_chain_info()?,
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

                            tx.send(DarwiniaCrabMessageReceive::FinishedInitBridge)
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
            BridgeName::DarwiniaToCrab => {
                type Source = component_darwinia_s2s::DarwiniaChain;
                type Target = component_crab_s2s::CrabChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    crab_runtime::BridgeGrandpaCall::<
                        crab_runtime::Runtime,
                        crab_runtime::WithDarwiniaGrandpa,
                    >::initialize(init_data)
                    .into()
                }

                $generic
            }
            BridgeName::CrabToDarwinia => {
                type Source = component_crab_s2s::CrabChain;
                type Target = component_darwinia_s2s::DarwiniaChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    darwinia_runtime::BridgeGrandpaCall::<
                        darwinia_runtime::Runtime,
                        darwinia_runtime::WithCrabGrandpa,
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
            target: DarwiniaCrabTask::NAME,
            "source client -> {:?}",
            source_client
        );
        log::debug!(
            target: DarwiniaCrabTask::NAME,
            "target client -> {:?}",
            target_client
        );

        let runtime_version = target_client.runtime_version().await?;
        substrate_relay_helper::headers_initialize::initialize(
            source_client,
            target_client.clone(),
            target_sign.public().into(),
            move |transaction_nonce, initialization_data| {
                Bytes(
                    Target::sign_transaction(SignParam {
                        spec_version: runtime_version.spec_version,
                        transaction_version: runtime_version.transaction_version,
                        genesis_hash: *target_client.genesis_hash(),
                        signer: target_sign.clone(),
                        era: relay_substrate_client::TransactionEra::immortal(),
                        unsigned: UnsignedTransaction::new(
                            encode_init_bridge(initialization_data),
                            transaction_nonce,
                        ),
                    })
                    .encode(),
                )
            },
        )
        .await;
    });
    Ok(())
}
