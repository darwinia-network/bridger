use abstract_bridge_s2s::config::Config;
use abstract_bridge_s2s::error::{S2SClientError, S2SClientResult};
use abstract_bridge_s2s::{
    client::{S2SClientBase, S2SClientGeneric},
    types::bp_header_chain,
};
use finality_grandpa::voter_set::VoterSet;
use sp_finality_grandpa::{AuthorityList, ConsensusLog, ScheduledChange};
use sp_runtime::{ConsensusEngineId, DigestItem};
use subxt::rpc::{ClientT, Subscription, SubscriptionClientT};
use subxt::{sp_core, sp_runtime};

use crate::client::PangoroClient;
use crate::error::{ClientError, ClientResult};
use crate::types::runtime_types::bp_header_chain::InitializationData;

const GRANDPA_ENGINE_ID: ConsensusEngineId = *b"FRNK";

pub(crate) type BundleHeader = crate::types::runtime_types::sp_runtime::generic::header::Header<
    u32,
    crate::types::runtime_types::sp_runtime::traits::BlakeTwo256,
>;
type SpHeader = sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>;

impl PangoroClient {
    async fn grandpa_authorities(&self, at: sp_core::H256) -> ClientResult<AuthorityList> {
        let params = subxt::rpc::rpc_params![
            "GrandpaApi_grandpa_authorities",
            sp_core::Bytes(Vec::new()),
            at
        ];
        let hex: String = self
            .subxt()
            .rpc()
            .client
            .request("state_call", params)
            .await?;
        let raw_authorities_set = array_bytes::hex2bytes(hex)?;
        let authorities = codec::Decode::decode(&mut &raw_authorities_set[..]).map_err(|err| {
            ClientError::Custom(format!(
                "[DecodeAuthorities] Can not decode authorities: {:?}",
                err
            ))
        })?;
        Ok(authorities)
    }

    /// Find header digest that schedules next GRANDPA authorities set.
    fn find_grandpa_authorities_scheduled_change(
        &self,
        header: &SpHeader,
    ) -> Option<ScheduledChange<u32>> {
        let filter_log = |log: ConsensusLog<u32>| match log {
            ConsensusLog::ScheduledChange(change) => Some(change),
            _ => None,
        };

        // find the first consensus digest with the right ID which converts to
        // the right kind of consensus log.
        header
            .digest
            .logs
            .iter()
            .filter_map(|item| match item {
                DigestItem::Consensus(engine, logs) => {
                    if engine == &GRANDPA_ENGINE_ID {
                        Some(&logs[..])
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .find_map(|mut l| {
                let log = codec::Decode::decode(&mut l).ok();
                log.and_then(filter_log)
            })
    }
}

impl S2SClientBase for PangoroClient {
    const CHAIN: &'static str = "pangoro";

    type Config = bp_pangoro::Pangoro;
    type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

#[async_trait::async_trait]
impl S2SClientGeneric for PangoroClient {
    type InitializationData = InitializationData<BundleHeader>;

    async fn subscribe_grandpa_justifications(
        &self,
    ) -> S2SClientResult<Subscription<sp_core::Bytes>> {
        Ok(self
            .subxt()
            .rpc()
            .client
            .subscribe(
                "grandpa_subscribeJustifications",
                None,
                "grandpa_unsubscribeJustifications",
            )
            .await?)
    }

    async fn prepare_initialization_data(&self) -> S2SClientResult<Self::InitializationData> {
        let mut subscription = self.subscribe_grandpa_justifications().await?;
        let justification = subscription
            .next()
            .await
            .ok_or_else(|| S2SClientError::Custom("The subscribe is closed".to_string()))??;
        let justification: bp_header_chain::justification::GrandpaJustification<SpHeader> =
            codec::Decode::decode(&mut &justification.0[..])
                .map_err(|err| S2SClientError::Custom(format!("Wrong justification: {:?}", err)))?;

        let (initial_header_hash, initial_header_number) = (
            justification.commit.target_hash,
            justification.commit.target_number,
        );
        let initial_header = self
            .subxt()
            .rpc()
            .header(Some(initial_header_hash))
            .await?
            .ok_or_else(|| {
                S2SClientError::Custom(format!(
                    "Can not get initial header by hash: {:?}",
                    initial_header_hash
                ))
            })?;
        tracing::trace!(target: "client-pangoro", "Selected initial header [{}]: {}",
            initial_header_number,
            initial_header_hash,
        );
        let initial_authorities_set = self.grandpa_authorities(initial_header_hash).await?;
        tracing::trace!(target: "client-pangoro", "Selected initial authorities set: {:?}",
            initial_authorities_set,
        );

        // If initial header changes the GRANDPA authorities set, then we need previous authorities
        // to verify justification.
        let mut authorities_for_verification = initial_authorities_set.clone();
        let scheduled_change = self.find_grandpa_authorities_scheduled_change(&initial_header);
        if scheduled_change
            .as_ref()
            .map(|c| c.delay == 0)
            .unwrap_or(false)
        {
            return Err(S2SClientError::Custom(format!(
                "GRANDPA authorities change at {} scheduled to happen in {:?} blocks. \
                We expect regular hange to have zero delay",
                initial_header_hash,
                scheduled_change.as_ref().map(|c| c.delay),
            )));
        }
        let schedules_change = scheduled_change.is_some();
        if schedules_change {
            authorities_for_verification =
                self.grandpa_authorities(initial_header.parent_hash).await?;
            tracing::trace!(
                target: "client-pangoro",
                "Selected header is scheduling GRANDPA authorities set changes. Using previous set: {:?}",
                authorities_for_verification,
            );
        }

        // Now let's try to guess authorities set id by verifying justification.
        let mut initial_authorities_set_id = 0;
        let mut min_possible_block_number = 0;
        let authorities_for_verification = VoterSet::new(authorities_for_verification.clone())
            .ok_or(S2SClientError::Custom(format!(
                "[ReadInvalidAuthorities]: {:?}",
                authorities_for_verification,
            )))?;
        loop {
            tracing::trace!(
                target: "client-pangoro",
                "Trying GRANDPA authorities set id: {}",
                initial_authorities_set_id,
            );

            let is_valid_set_id = bp_header_chain::justification::verify_justification::<SpHeader>(
                (initial_header_hash, initial_header_number),
                initial_authorities_set_id,
                &authorities_for_verification,
                &justification,
            )
            .is_ok();

            if is_valid_set_id {
                break;
            }

            initial_authorities_set_id += 1;
            min_possible_block_number += 1;
            if min_possible_block_number > initial_header_number {
                // there can't be more authorities set changes than headers => if we have reached
                // `initial_block_number` and still have not found correct value of
                // `initial_authorities_set_id`, then something else is broken => fail
                return Err(S2SClientError::Custom(format!(
                    "[GuessInitialAuthorities]: {}",
                    initial_header_number
                )));
            }
        }

        let initialization_data = bp_header_chain::InitializationData {
            header: Box::new(initial_header),
            authority_list: initial_authorities_set,
            set_id: if schedules_change {
                initial_authorities_set_id + 1
            } else {
                initial_authorities_set_id
            },
            is_halted: false,
        };
        let bytes = codec::Encode::encode(&initialization_data);
        Ok(codec::Decode::decode(&mut &bytes[..]).map_err(|e| {
            S2SClientError::Custom(format!("Failed to decode initialization data: {:?}", e))
        })?)
    }

    async fn initialize(
        &self,
        initialization_data: Self::InitializationData,
    ) -> S2SClientResult<<Self::Config as Config>::Hash> {
        let hash = self
            .runtime()
            .tx()
            .bridge_pangolin_grandpa()
            .initialize(initialization_data)
            .sign_and_submit(self.account().signer())
            .await?;
        Ok(hash)
    }
}
