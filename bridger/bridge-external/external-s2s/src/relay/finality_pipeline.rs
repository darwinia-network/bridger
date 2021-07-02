//! Substrate-to-Substrate headers sync entrypoint.

use std::{fmt::Debug, marker::PhantomData, time::Duration};

use bp_header_chain::justification::GrandpaJustification;
use finality_relay::{FinalitySyncParams, FinalitySyncPipeline};
use relay_substrate_client::{
    finality_source::FinalitySource, BlockNumberOf, Chain, Client, HashOf, SyncHeader,
};
use relay_utils::{metrics::MetricsParams, BlockNumberBase};
use sp_core::Bytes;

use crate::relay::finality_target::SubstrateFinalityTarget;

/// Default synchronization loop timeout.
pub(crate) const STALL_TIMEOUT: Duration = Duration::from_secs(120);
/// Default limit of recent finality proofs.
///
/// Finality delay of 4096 blocks is unlikely to happen in practice in
/// Substrate+GRANDPA based chains (good to know).
pub(crate) const RECENT_FINALITY_PROOFS_LIMIT: usize = 4096;

/// Headers sync pipeline for Substrate <-> Substrate relays.
pub trait SubstrateFinalitySyncPipeline: FinalitySyncPipeline {
    /// Name of the runtime method that returns id of best finalized source header at target chain.
    const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str;

    /// Chain with GRANDPA bridge pallet.
    type TargetChain: Chain;

    /// Customize metrics exposed by headers sync loop.
    fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
        Ok(params)
    }

    /// Start finality relay guards.
    ///
    /// Different finality bridges may have different set of guards - e.g. on ephemeral chains we
    /// don't need version guards, on test chains we don't care that much about relayer account
    /// balance, ... So the implementation is left to the specific bridges.
    fn start_relay_guards(_target_client: &Client<Self::TargetChain>) {}

    /// Returns id of account that we're using to sign transactions at target chain.
    fn transactions_author(&self) -> <Self::TargetChain as Chain>::AccountId;

    /// Make submit header transaction.
    fn make_submit_finality_proof_transaction(
        &self,
        transaction_nonce: <Self::TargetChain as Chain>::Index,
        header: Self::Header,
        proof: Self::FinalityProof,
    ) -> Bytes;
}

/// Substrate-to-Substrate finality proof pipeline.
#[derive(Clone)]
pub struct SubstrateFinalityToSubstrate<SourceChain, TargetChain: Chain, TargetSign> {
    /// Client for the target chain.
    pub(crate) target_client: Client<TargetChain>,
    /// Data required to sign target chain transactions.
    pub(crate) target_sign: TargetSign,
    /// Unused generic arguments dump.
    _marker: PhantomData<SourceChain>,
}

impl<SourceChain, TargetChain: Chain, TargetSign> Debug
    for SubstrateFinalityToSubstrate<SourceChain, TargetChain, TargetSign>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubstrateFinalityToSubstrate")
            .field("target_client", &self.target_client)
            .finish()
    }
}

impl<SourceChain, TargetChain: Chain, TargetSign>
    SubstrateFinalityToSubstrate<SourceChain, TargetChain, TargetSign>
{
    /// Create new Substrate-to-Substrate headers pipeline.
    pub fn new(target_client: Client<TargetChain>, target_sign: TargetSign) -> Self {
        SubstrateFinalityToSubstrate {
            target_client,
            target_sign,
            _marker: Default::default(),
        }
    }
}

impl<SourceChain, TargetChain, TargetSign> FinalitySyncPipeline
    for SubstrateFinalityToSubstrate<SourceChain, TargetChain, TargetSign>
where
    SourceChain: Clone + Chain + Debug,
    BlockNumberOf<SourceChain>: BlockNumberBase,
    TargetChain: Clone + Chain + Debug,
    TargetSign: 'static + Clone + Send + Sync,
{
    const SOURCE_NAME: &'static str = SourceChain::NAME;
    const TARGET_NAME: &'static str = TargetChain::NAME;

    type Hash = HashOf<SourceChain>;
    type Number = BlockNumberOf<SourceChain>;
    type Header = SyncHeader<SourceChain::Header>;
    type FinalityProof = GrandpaJustification<SourceChain::Header>;
}

/// Run Substrate-to-Substrate finality sync.
pub async fn run<SourceChain, TargetChain, P>(
    pipeline: P,
    source_client: Client<SourceChain>,
    target_client: Client<TargetChain>,
    metrics_params: MetricsParams,
) -> anyhow::Result<()>
where
    P: SubstrateFinalitySyncPipeline<
        Hash = HashOf<SourceChain>,
        Number = BlockNumberOf<SourceChain>,
        Header = SyncHeader<SourceChain::Header>,
        FinalityProof = GrandpaJustification<SourceChain::Header>,
        TargetChain = TargetChain,
    >,
    SourceChain: Clone + Chain,
    BlockNumberOf<SourceChain>: BlockNumberBase,
    TargetChain: Clone + Chain,
{
    log::info!(
        target: "bridge",
        "Starting {} -> {} finality proof relay",
        SourceChain::NAME,
        TargetChain::NAME,
    );

    finality_relay::run(
        FinalitySource::new(source_client, None),
        SubstrateFinalityTarget::new(target_client, pipeline),
        FinalitySyncParams {
            tick: std::cmp::max(
                SourceChain::AVERAGE_BLOCK_INTERVAL,
                TargetChain::AVERAGE_BLOCK_INTERVAL,
            ),
            recent_finality_proofs_limit: RECENT_FINALITY_PROOFS_LIMIT,
            stall_timeout: STALL_TIMEOUT,
        },
        metrics_params,
        futures::future::pending(),
    )
    .await
    .map_err(|e| anyhow::format_err!("{}", e))
}
