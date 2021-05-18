use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use relay_substrate_client::{Chain as RelaySubstrateClientChain, TransactionSignScheme};
use sp_core::{Bytes, Pair};

use crate::types::transfer::ChainInfo;

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			("pangolin", "millau") => {
				type Source = pangolin_bridge_relay_client_definition::PangolinChain;
				type Target = relay_millau_client::Millau;

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
			_ => {
				anyhow::bail!(
					"Not support bridge {} -> {}",
					$bridge.0.to_string(),
					$bridge.1.to_string(),
				);
			}
		}
	};
}

pub async fn run(source_chain: ChainInfo, target_chain: ChainInfo) -> anyhow::Result<()> {
	let bridge = (&source_chain.name()[..], &target_chain.name()[..]);
	info!("Init bridge {} -> {}", bridge.0, bridge.1);
	debug!("source -> {:?}", source_chain);
	debug!("target -> {:?}", target_chain);
	select_bridge!(bridge, {
		let source_client = source_chain.to_substrate_relay_chain::<Source>().await?;
		let target_client = target_chain.to_substrate_relay_chain::<Target>().await?;
		let target_sign = target_chain.to_keypair::<Target>()?;
		debug!("source client -> {:?}", source_client);
		debug!("target client -> {:?}", target_client);

		crate::types::s2s::headers_initialize::initialize(
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
