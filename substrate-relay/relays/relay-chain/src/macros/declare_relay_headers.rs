#[macro_export]
macro_rules! declare_relay_headers {
	(
		/// source chain name
		$source_name:ident,
		/// target chain name
		$target_name:ident,
		/// source relay chain
		$source_relay_chain:ident,
		/// target relay chain
		$target_relay_chain:ident,
		/// source relay client module
		$source_relay_client:ident,
		/// source chain const
		$source_const:ident,
		/// source chain bridge primitives
		$source_primitives:ident,
		/// target chain bridge primitives
		$target_primitives:ident,
		/// target chain runtime
		$target_runtime:ident,
		/// target chain to source chain grandpa pallet
		$source_grandpa_pallet_in_target:ident,
		/// target chain to source chain grandpa instance
		$source_grandpa_instance_in_target:ident,
	) => {
		paste::item! {
			/// Source-to-Target finality sync pipeline.
			pub(crate) type [<$source_name FinalityTo $target_name>] = SubstrateFinalityToSubstrate<
				$source_relay_chain,
				$target_relay_chain,
				<$source_const as ChainConst>::SigningParams,
			>;

			impl SubstrateFinalitySyncPipeline for [<$source_name FinalityTo $target_name>] {
				const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = $source_const::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;

				type TargetChain = $target_relay_chain;

				fn transactions_author(&self) -> $target_primitives::AccountId {
					(*self.target_sign.public().as_array_ref()).into()
				}

				fn make_submit_finality_proof_transaction(
					&self,
					transaction_nonce: <$target_relay_chain as relay_substrate_client::Chain>::Index,
					header: $source_relay_client::SyncHeader,
					proof: GrandpaJustification<$source_primitives::Header>,
				) -> Bytes {
					let call = $source_grandpa_pallet_in_target::<
						$target_runtime::Runtime,
						$source_grandpa_instance_in_target,
					>::submit_finality_proof(header.into_inner(), proof)
					.into();

					let genesis_hash = *self.target_client.genesis_hash();
					let transaction = $target_relay_chain::sign_transaction(genesis_hash, &self.target_sign, transaction_nonce, call);

					Bytes(transaction.encode())
				}

			}
		}
	};
}
