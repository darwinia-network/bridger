use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use relay_substrate_client::{
    Chain as RelaySubstrateClientChain, SignParam, TransactionSignScheme, UnsignedTransaction,
};
use sp_core::{Bytes, Pair};

use support_common::config::{Config, Names};
use support_common::error::BridgerError;
use support_terminal::output;

use crate::bridge::{ChainInfoConfig, DarwiniaCrabConfig};
use crate::types::{BridgeName, InitBridge};

pub async fn handle_init(bridge: BridgeName) -> color_eyre::Result<()> {
    tracing::info!(target: "darwinia-crab", "Init bridge {:?}", bridge);
    let bridge_config: DarwiniaCrabConfig = Config::restore(Names::BridgeDarwiniaCrab)?;
    let config_darwinia: ChainInfoConfig = bridge_config.darwinia;
    let config_crab: ChainInfoConfig = bridge_config.crab;

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
    .map_err(|_| BridgerError::Custom("Failed to join thread handle".to_string()))??;

    output::output_ok();
    Ok(())
}

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            BridgeName::DarwiniaToCrab => {
                type Source = client_darwinia::DarwiniaChain;
                type Target = client_crab::CrabChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    crab_runtime::BridgeGrandpaCall::<
                        crab_runtime::Runtime,
                        crab_runtime::WithDarwiniaGrandpa,
                    >::initialize {
                        init_data,
                    }
                    .into()
                }

                $generic
            }
            BridgeName::CrabToDarwinia => {
                type Source = client_crab::CrabChain;
                type Target = client_darwinia::DarwiniaChain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    darwinia_runtime::BridgeGrandpaCall::<
                        darwinia_runtime::Runtime,
                        darwinia_runtime::WithCrabGrandpa,
                    >::initialize {
                        init_data,
                    }
                    .into()
                }

                $generic
            }
        }
    };
}

async fn init_bridge(init_bridge: InitBridge) -> color_eyre::Result<()> {
    let bridge = init_bridge.bridge;
    let source_chain = init_bridge.source;
    let target_chain = init_bridge.target;
    select_bridge!(bridge, {
        let source_client = source_chain.to_substrate_relay_chain::<Source>().await?;
        let target_client = target_chain.to_substrate_relay_chain::<Target>().await?;
        let target_sign = target_chain.to_keypair::<Target>()?;
        tracing::debug!(
            target: "darwinia-crab",
            "source client -> {:?}",
            source_client
        );
        tracing::debug!(
            target: "darwinia-crab",
            "target client -> {:?}",
            target_client
        );

        let (spec_version, transaction_version) = target_client.simple_runtime_version().await?;
        substrate_relay_helper::headers_initialize::initialize(
            source_client,
            target_client.clone(),
            target_sign.public().into(),
            move |transaction_nonce, initialization_data| {
                Bytes(
                    Target::sign_transaction(SignParam {
                        spec_version,
                        transaction_version,
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
