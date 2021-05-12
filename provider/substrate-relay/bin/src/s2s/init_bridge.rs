use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use relay_chain::*;
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};

use crate::error;
use crate::persist::Chain;

macro_rules! select_bridge {
    ($bridge: expr, $generic: tt) => {
        match $bridge {
            ("pangolin", "millau") => {
                type Source = <RelayChainPangolin as RelayChain>::Chain;
                type Target = <RelayChainMillau as RelayChain>::Chain;

                fn encode_init_bridge(
                    init_data: InitializationData<<Source as ChainBase>::Header>,
                ) -> <Target as RelaySubstrateClientChain>::Call {
                    let initialize_call = millau_runtime::BridgeGrandpaPangolinCall::<
                        millau_runtime::Runtime,
                        millau_runtime::WithPangolinGrandpaInstance,
                    >::initialize(init_data);
                    millau_runtime::SudoCall::sudo(Box::new(initialize_call.into())).into()
                }

                $generic
            }
            _ => {
                return Err(error::CliError::NotSupportBridge(
                    $bridge.0.to_string(),
                    $bridge.1.to_string(),
                ))?
            }
        }
    };
}

pub async fn init(source_chain: &Chain, target_chain: &Chain) -> error::Result<()> {
    let bridge = (&source_chain.name()[..], &target_chain.name()[..]);
    select_bridge!(bridge, {
        let source_client = source_chain.to_substrate_relay_chain::<Source>().await?;
        let target_client = target_chain.to_substrate_relay_chain::<Target>().await?;
        let target_sign = target_chain.to_keypair::<Target>()?;

        crate::s2s::lib::headers_initialize::initialize(
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
