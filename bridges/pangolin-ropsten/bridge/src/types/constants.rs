/// Graphql query constants
pub mod graphql_query {
    /// Authorities change signed event
    pub const AUTHORITIES_CHANGE_SIGNED_EVENT: &str =
        include_str!("../graphql/authorities_change_signed_event.query.graphql");
    /// Latest shedule mmr_root event
    pub const LATEST_SCHEDULE_MMR_ROOT_EVENT: &str =
        include_str!("../graphql/latest_schedule_mmr_root_event.query.graphql");
    /// mmr_root signed events
    pub const MMR_ROOT_SIGNED_EVENTS: &str =
        include_str!("../graphql/mmr_root_signed_events.query.graphql");
    /// Schedule authorities change event
    pub const SCHEDULE_AUTHORITIES_CHANGE_EVENT: &str =
        include_str!("../graphql/schedule_authorities_change_event.query.graphql");
}
