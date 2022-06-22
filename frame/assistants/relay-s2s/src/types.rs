use abstract_client_s2s::client::{
    S2SClientRelay, S2SParaBridgeClientRelaychain, S2SParaBridgeClientSolochain,
};
use abstract_client_s2s::config::Config;
use subquery_s2s::types::OriginType;
use subquery_s2s::Subquery;

use crate::error::{RelayError, RelayResult};

pub type LaneId = [u8; 4];

pub struct SolochainHeaderInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub lanes: Vec<LaneId>,
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub index_origin_type: OriginType,
}

pub struct RelaychainHeaderInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub client_relaychain: SC,
    pub client_solochain: TC,
    pub subquery_relaychain: Subquery,
    pub subquery_parachain: Subquery,
    pub index_origin_type: OriginType,
    // todo: merge this subquery to subquery_relaychain
    pub subquery_candidate: subquery_parachain::Subquery,
}

pub struct ParaHeaderInput<SC: S2SParaBridgeClientRelaychain, TC: S2SParaBridgeClientSolochain> {
    pub client_relaychain: SC,
    pub client_solochain: TC,
    pub para_id: u32,
}

pub struct JustificationInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub client_source: SC,
    pub client_target: TC,
}

pub struct MessageInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub lanes: Vec<LaneId>,
    pub relayer_account: <SC::Config as Config>::AccountId,
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub subquery_target: Subquery,
}

impl<SC: S2SClientRelay, TC: S2SClientRelay> MessageInput<SC, TC> {
    pub fn lane(&self) -> RelayResult<LaneId> {
        self.lanes
            .clone()
            .get(0)
            .cloned()
            .ok_or_else(|| RelayError::Custom("Missing lane id".to_string()))
    }
}
