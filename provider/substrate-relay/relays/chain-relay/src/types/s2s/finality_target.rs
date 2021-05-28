//! Substrate client as Substrate finality proof target. The chain we connect to should have
//! runtime that implements `<BridgedChainName>FinalityApi` to allow bridging with
//! <BridgedName> chain.

use crate::types::s2s::finality_pipeline::SubstrateFinalitySyncPipeline;

use async_trait::async_trait;
use codec::Decode;
use finality_relay::TargetClient;
use relay_substrate_client::{Chain, Client, Error as SubstrateError};
use relay_utils::relay_loop::Client as RelayClient;

/// Substrate client as Substrate finality target.
pub struct SubstrateFinalityTarget<C: Chain, P> {
	client: Client<C>,
	pipeline: P,
}

impl<C: Chain, P> SubstrateFinalityTarget<C, P> {
	/// Create new Substrate headers target.
	pub fn new(client: Client<C>, pipeline: P) -> Self {
		SubstrateFinalityTarget { client, pipeline }
	}
}

impl<C: Chain, P: SubstrateFinalitySyncPipeline> Clone for SubstrateFinalityTarget<C, P> {
	fn clone(&self) -> Self {
		SubstrateFinalityTarget {
			client: self.client.clone(),
			pipeline: self.pipeline.clone(),
		}
	}
}

#[async_trait]
impl<C: Chain, P: SubstrateFinalitySyncPipeline> RelayClient for SubstrateFinalityTarget<C, P> {
	type Error = SubstrateError;

	async fn reconnect(&mut self) -> Result<(), SubstrateError> {
		self.client.reconnect().await
	}
}

#[async_trait]
impl<C, P> TargetClient<P> for SubstrateFinalityTarget<C, P>
where
	C: Chain,
	P::Number: Decode,
	P::Hash: Decode,
	P: SubstrateFinalitySyncPipeline<TargetChain = C>,
{
	async fn best_finalized_source_block_number(&self) -> Result<P::Number, SubstrateError> {
		// we can't continue to relay finality if target node is out of sync, because
		// it may have already received (some of) headers that we're going to relay
		self.client.ensure_synced().await?;

		Ok(
			crate::types::s2s::messages_source::read_client_state::<C, P::Hash, P::Number>(
				&self.client,
				P::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET,
			)
			.await?
			.best_finalized_peer_at_best_self
			.0,
		)
	}

	async fn submit_finality_proof(
		&self,
		header: P::Header,
		proof: P::FinalityProof,
	) -> Result<(), SubstrateError> {
		self.client
			.submit_signed_extrinsic(
				self.pipeline.transactions_author(),
				move |transaction_nonce| {
					self.pipeline.make_submit_finality_proof_transaction(
						transaction_nonce,
						header,
						proof,
					)
				},
			)
			.await
			.map(drop)
	}
}
