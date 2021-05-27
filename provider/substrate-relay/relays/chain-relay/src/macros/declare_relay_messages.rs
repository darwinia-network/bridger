#[macro_export]
macro_rules! declare_relay_messages {
	(
		/// source chain name
		$source_name:ident,
		/// target chain name
		$target_name:ident,
		/// source relay chain
		$source_relay_chain:ident,
		/// target relay chain
		$target_relay_chain:ident,
		/// source chain relay client module
		$source_relay_client:ident,
		/// target chain relay client module
		$target_relay_client:ident,
		/// source chain const
		$source_const:ident,
		/// target chain const
		$target_const:ident,
		/// source chain bridge primitives
		$source_primitives:ident,
		/// target chain bridge primitives
		$target_primitives:ident,
		/// source chain runtime
		$source_runtime:ident,
		/// target chain runtime
		$target_runtime:ident,
		/// source chain messages pallet call
		$source_messages_pallet:ident,
		/// target chain messages pallet call
		$target_messages_pallet:ident,
		/// source chain extrinsic weight module, (include `max_extrinsic_weight`, `max_extrinsic_size` and more functions, types)
		$source_extrinsic_weight_module:ident,
		/// target chain extrinsic weight module, (include `max_extrinsic_weight`, `max_extrinsic_size` and more functions, types)
		$target_extrinsic_weight_module:ident,
		/// source messages instance in target chain
		$source_messages_instance_in_target_chain:ident,
		/// target messages instance in source chain
		$target_messages_instance_in_source_chain:ident,
		/// target to source conversion rate. Initially we treat both tokens as equal.
		$target_source_conversion_rate_in_source_chain:ident,
		/// target to source conversion rate. default value
		$target_source_conversion_rate_default_in_source_chain:ident,
	) => {
		paste::item! {


			/// Source-to-Target message lane.
			pub type [<$source_name MessagesTo $target_name>] = SubstrateMessageLaneToSubstrate<
				$source_relay_chain,
				<$source_const as ChainConst>::SigningParams,
				$target_relay_chain,
				<$target_const as ChainConst>::SigningParams,
			>;

			impl SubstrateMessageLane for [<$source_name MessagesTo $target_name>] {
				const OUTBOUND_LANE_MESSAGES_DISPATCH_WEIGHT_METHOD: &'static str =
					$target_const::OUTBOUND_LANE_MESSAGES_DISPATCH_WEIGHT_METHOD;
				const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
					$target_const::OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD;
				const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
					$target_const::OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;

				const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
					$source_const::INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD;
				const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
					$source_const::INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD;
				const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
					$source_const::INBOUND_LANE_UNREWARDED_RELAYERS_STATE;

				const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
					$source_const::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET;
				const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
					$target_const::BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE;

				type SourceChain = $source_relay_chain;
				type TargetChain = $target_relay_chain;

				fn source_transactions_author(&self) -> $source_primitives::AccountId {
					(*self.source_sign.public().as_array_ref()).into()
				}

				fn make_messages_receiving_proof_transaction(
					&self,
					transaction_nonce: <$source_relay_chain as relay_substrate_client::Chain>::Index,
					_generated_at_block: $target_relay_client::HeaderId,
					proof: <Self as MessageLane>::MessagesReceivingProof,
				) -> Bytes {
					let _source_name = String::from(stringify!($source_name));
					let _target_name = String::from(stringify!($target_name));

					let (relayers_state, proof) = proof;
					let call: $source_runtime::Call = $source_messages_pallet::receive_messages_delivery_proof::<
						$source_runtime::Runtime,
						$target_messages_instance_in_source_chain,
					>(
						proof,
						relayers_state
					).into();
					let call_weight = call.get_dispatch_info().weight;
					let genesis_hash = *self.source_client.genesis_hash();
					let transaction = $source_relay_chain::sign_transaction(
						genesis_hash,
						&self.source_sign,
						transaction_nonce,
						call
					);
					log::trace!(
						target: "bridge",
						"Prepared {} -> {} confirmation transaction. Weight: {}/{}, size: {}/{}",
						_target_name,
						_source_name,
						call_weight,
						$source_extrinsic_weight_module::max_extrinsic_weight(),
						transaction.encode().len(),
						$source_extrinsic_weight_module::max_extrinsic_size(),
					);
					Bytes(transaction.encode())
				}


				fn target_transactions_author(&self) -> $target_primitives::AccountId {
					(*self.target_sign.public().as_array_ref()).into()
				}

				fn make_messages_delivery_transaction(
					&self,
					transaction_nonce: <$target_relay_chain as relay_substrate_client::Chain>::Index,
					_generated_at_header: $source_relay_client::HeaderId,
					_nonces: RangeInclusive<MessageNonce>,
					proof: <Self as MessageLane>::MessagesProof,
				) -> Bytes {
					let _source_name = String::from(stringify!($source_name));
					let _target_name = String::from(stringify!($target_name));

					let (dispatch_weight, proof) = proof;
					let FromBridgedChainMessagesProof {
						ref nonces_start,
						ref nonces_end,
						..
					} = proof;
					let messages_count = nonces_end - nonces_start + 1;
					let call: $target_runtime::Call = $target_messages_pallet::receive_messages_proof::<
						$target_runtime::Runtime,
						$source_messages_instance_in_target_chain,
					>(
						self.relayer_id_at_source.clone(),
						proof,
						messages_count as _,
						dispatch_weight,
					).into();
					let call_weight = call.get_dispatch_info().weight;
					let genesis_hash = *self.target_client.genesis_hash();
					let transaction = $target_relay_chain::sign_transaction(
						genesis_hash,
						&self.target_sign,
						transaction_nonce,
						call
					);
					log::trace!(
						target: "bridge",
						"Prepared {} -> {} delivery transaction. Weight: {}/{}, size: {}/{}",
						_source_name,
						_target_name,
						call_weight,
						$target_extrinsic_weight_module::max_extrinsic_weight(),
						transaction.encode().len(),
						$target_extrinsic_weight_module::max_extrinsic_size(),
					);
					Bytes(transaction.encode())
				}
			}

			/// Source node as messages source.
			type [<$source_name SourceClient>] = SubstrateMessagesSource<
				$source_relay_chain,
				[<$source_name MessagesTo $target_name>],
				$source_runtime::Runtime,
				$target_messages_instance_in_source_chain,
			>;

			/// Target node as messages target.
			type [<$target_name TargetClient>] = SubstrateMessagesTarget<
				$target_relay_chain,
				[<$source_name MessagesTo $target_name>],
				$target_runtime::Runtime,
				$source_messages_instance_in_target_chain,
			>;

			pub struct [<$source_name MessagesTo $target_name Runner>];

			#[allow(non_snake_case)]
			impl [<$source_name MessagesTo $target_name Runner>] {
				pub async fn run(
					params: MessagesRelayParams<
						$source_relay_chain,
						<$source_const as ChainConst>::SigningParams,
						$target_relay_chain,
						<$target_const as ChainConst>::SigningParams,
					>,
				) -> Result<(), String> {
					let _source_name = String::from(stringify!($source_name));
					let _target_name = String::from(stringify!($target_name));

					let stall_timeout = Duration::from_secs(5 * 60);
					let [<relayer_id_at_ $source_name>] = (*params.source_sign.public().as_array_ref()).into();

					let lane_id = params.lane_id;
					let source_client = params.source_client;
					let lane = [<$source_name MessagesTo $target_name>] {
						source_client: source_client.clone(),
						source_sign: params.source_sign,
						target_client: params.target_client.clone(),
						target_sign: params.target_sign,
						relayer_id_at_source: [<relayer_id_at_ $source_name>],
					};

					// 2/3 is reserved for proofs and tx overhead
					let max_messages_size_in_single_batch = $target_extrinsic_weight_module::max_extrinsic_size() as usize / 3;
					let (
						max_messages_in_single_batch,
						max_messages_weight_in_single_batch
					) = select_delivery_transaction_limits::<
							// todo: there can be change to special weight
							pallet_bridge_messages::weights::RialtoWeight<$source_runtime::Runtime>
						>(
							$target_extrinsic_weight_module::max_extrinsic_weight(),
							$target_const::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
						);

					log::info!(
						target: "bridge",
						"Starting {} -> {} messages relay.\n\t\
{} relayer account id: {:?}\n\t\
Max messages in single transaction: {}\n\t\
Max messages size in single transaction: {}\n\t\
Max messages weight in single transaction: {}",
						_source_name,
						_target_name,
						_source_name,
						lane.relayer_id_at_source,
						max_messages_in_single_batch,
						max_messages_size_in_single_batch,
						max_messages_weight_in_single_batch,
					);


					messages_relay::message_lane_loop::run(
						messages_relay::message_lane_loop::Params {
							lane: lane_id,
							source_tick: $source_const::AVERAGE_BLOCK_INTERVAL,
							target_tick: $target_const::AVERAGE_BLOCK_INTERVAL,
							reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
							stall_timeout,
							delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
								max_unrewarded_relayer_entries_at_target: $target_const::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
								max_unconfirmed_nonces_at_target: $target_const::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
								max_messages_in_single_batch,
								max_messages_weight_in_single_batch,
								max_messages_size_in_single_batch,
							},
						},
						[<$source_name SourceClient>]::new(
							source_client.clone(),
							lane.clone(),
							lane_id,
							$target_const::BRIDGE_CHAIN_ID,
							params.target_to_source_headers_relay,
						),
						[<$target_name TargetClient>]::new(
							params.target_client,
							lane,
							lane_id,
							$source_const::BRIDGE_CHAIN_ID,
							params.source_to_target_headers_relay,
						),
						relay_utils::relay_metrics(
							Some(messages_relay::message_lane_loop::metrics_prefix::<
								[<$source_name MessagesTo $target_name>],
							>(&lane_id)),
							params.metrics_params,
						)
						.standalone_metric(|registry, prefix| {
							StorageProofOverheadMetric::new(
								registry,
								prefix,
								source_client.clone(),
								format!("{}_storage_proof_overhead", _source_name),
								format!("{} storage proof overhead", _source_name),
							)
						})?
						.standalone_metric(|registry, prefix| {
							FloatStorageValueMetric::<_, sp_runtime::FixedU128>::new(
								registry,
								prefix,
								source_client,
								sp_core::storage::StorageKey(
									$target_source_conversion_rate_in_source_chain::key().to_vec(),
								),
								Some($target_source_conversion_rate_default_in_source_chain),
								format!("{}_{}_to_{}_conversion_rate", _source_name, _target_name, _source_name),
								format!("{} to {} tokens conversion rate (used by {})", _target_name, _source_name, _target_name),
							)
						})?
						.into_params(),
						futures::future::pending(),
					).await

				}
			}

		}
	};
}
