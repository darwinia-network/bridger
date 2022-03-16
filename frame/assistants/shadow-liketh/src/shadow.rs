use crate::config::ShadowConfig;

/// Shadow client
pub struct Shadow {
    endpoint: String,
    gql: gql_client::Client,
}

impl Shadow {
    /// Create shadow instance
    pub fn new(endpoint: String, gql: gql_client::Client) -> Self {
        Self { endpoint, gql }
    }
}

impl Shadow {
    //
    pub fn mmr_root(&self, leaf_index: u64) {
        let position = mmr_client::mmr::leaf_index_to_pos(leaf_index);
        let peak_positions = mmr_client::mmr::get_peaks(position);
        println!("{:?}", peak_positions);
    }
}
