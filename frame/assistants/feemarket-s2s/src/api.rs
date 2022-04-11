use relay_substrate_client::{Chain, Client};

/// Feemarket api
pub struct FeemarketApi<C: Chain> {
    client: Client<C>,
}

impl<C: Chain> FeemarketApi<C> {
    pub fn new(client: Client<C>) -> Self {
        Self { client }
    }
}
