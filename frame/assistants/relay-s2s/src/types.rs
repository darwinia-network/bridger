use bridge_s2s_traits::client::{S2SClientGeneric, S2SClientRelay};
#[cfg(feature = "bridge-parachain")]
use bridge_s2s_traits::client::{S2SParaBridgeClientRelaychain, S2SParaBridgeClientSolochain};
use bridge_s2s_traits::strategy::RelayStrategy;
use bridge_s2s_traits::types::bp_runtime::Chain;

use subquery::types::OriginType;
use subquery::Subquery;

use crate::error::{RelayError, RelayResult};

pub(crate) static M_HEADER: &str = "header";
#[cfg(feature = "bridge-parachain")]
pub(crate) static M_PARA_HEAD: &str = "para-head";
pub(crate) static M_DELIVERY: &str = "delivery";
pub(crate) static M_RECEIVING: &str = "receiving";

pub type LaneId = [u8; 4];

pub struct SolochainHeaderInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub index_origin_type: OriginType,
    pub enable_mandatory: bool,
}

#[cfg(feature = "bridge-parachain")]
pub struct RelaychainHeaderInput<SC: S2SClientGeneric, TC: S2SClientRelay> {
    pub client_relaychain: SC,
    pub client_solochain: TC,
    pub subquery_relaychain: Subquery,
    pub subquery_parachain: Subquery,
    pub index_origin_type: OriginType,
    pub enable_mandatory: bool,
}

#[cfg(feature = "bridge-parachain")]
pub struct ParaHeaderInput<SC: S2SParaBridgeClientRelaychain, TC: S2SParaBridgeClientSolochain> {
    pub client_relaychain: SC,
    pub client_solochain: TC,
    pub para_id: u32,
}

pub struct JustificationInput<C: S2SClientGeneric> {
    pub client: C,
}

pub struct MessageDeliveryInput<SC: S2SClientRelay, TC: S2SClientRelay, Strategy: RelayStrategy> {
    pub lanes: Vec<LaneId>,
    pub nonces_limit: u64,
    pub relayer_account: <SC::Chain as Chain>::AccountId,
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub subquery_target: Subquery,
    pub relay_block_origin: OriginType,
    pub relay_strategy: Strategy,
}

// impl<SC: S2SClientRelay, TC: S2SClientRelay, Strategy: RelayStrategy>
//     MessageDeliveryInput<SC, TC, Strategy>
// {
//     // todo: support multiple lanes
//     pub fn lane(&self) -> RelayResult<LaneId> {
//         self.lanes
//             .clone()
//             .get(0)
//             .cloned()
//             .ok_or_else(|| RelayError::Custom("Missing lane id".to_string()))
//     }
// }

pub struct MessageReceivingInput<SC: S2SClientRelay, TC: S2SClientRelay> {
    pub lanes: Vec<LaneId>,
    pub relayer_account: <SC::Chain as Chain>::AccountId,
    pub client_source: SC,
    pub client_target: TC,
    pub subquery_source: Subquery,
    pub subquery_target: Subquery,
}

// impl<SC: S2SClientRelay, TC: S2SClientRelay> MessageReceivingInput<SC, TC> {
//     // todo: support multiple lanes
//     pub fn lane(&self) -> RelayResult<LaneId> {
//         self.lanes
//             .clone()
//             .get(0)
//             .cloned()
//             .ok_or_else(|| RelayError::Custom("Missing lane id".to_string()))
//     }
// }
