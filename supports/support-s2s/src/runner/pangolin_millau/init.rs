#![allow(unreachable_patterns)]
use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};

use crate::types::{BridgeName, InitBridge};

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            BridgeName::PangolinToMillau => {
                type Source = component_pangolin::PangolinChain;
                type Target = component_millau::MillauChain;

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
                type Source = component_millau::MillauChain;
                type Target = component_pangolin::PangolinChain;

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
            _ => anyhow::bail!("Not support bridge {:?}", $bridge),
        }
    };
}

pub async fn init_bridge(init_bridge: InitBridge) -> anyhow::Result<()> {
    let bridge = init_bridge.bridge;
    let source_chain = init_bridge.source;
    let target_chain = init_bridge.target;
    select_bridge!(bridge, {
        let source_client = source_chain.to_substrate_relay_chain::<Source>().await?;
        let target_client = target_chain.to_substrate_relay_chain::<Target>().await?;
        let target_sign = target_chain.to_keypair::<Target>()?;
        log::debug!("source client -> {:?}", source_client);
        log::debug!("target client -> {:?}", target_client);

        crate::relay::headers_initialize::initialize(
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
    Ok(())
}
